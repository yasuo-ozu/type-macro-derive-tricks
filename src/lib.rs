#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
use syn::{
    parse_macro_input, punctuated::Punctuated, Data, DeriveInput, Fields, Generics, Ident, Type,
};
use template_quote::quote;

/// Main procedural macro that handles types with macros in type positions
///
/// Usage: `#[macro_derive(Trait1, Trait2, ...)]`
///
/// This macro:
/// 1. Identifies all macro invocations in type positions
/// 2. Generates unique type aliases for each macro type
/// 3. Replaces the macro types with the aliases
/// 4. Applies the specified derive traits to the transformed type
#[proc_macro_attribute]
pub fn macro_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    let derive_traits = parse_derive_traits(args);
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = impl_type_macro_derive_tricks(&derive_traits, &input);
    TokenStream::from(expanded)
}

fn parse_derive_traits(args: TokenStream) -> Vec<syn::Path> {
    let args = TokenStream2::from(args);

    if args.is_empty() {
        return Vec::new();
    }

    // Parse comma-separated list of trait names
    let mut traits = Vec::new();
    let mut current_trait = String::new();

    for token in args.into_iter() {
        match token {
            proc_macro2::TokenTree::Punct(punct) if punct.as_char() == ',' => {
                if !current_trait.is_empty() {
                    if let Ok(path) = syn::parse_str::<syn::Path>(current_trait.trim()) {
                        traits.push(path);
                    }
                    current_trait.clear();
                }
            }
            _ => {
                current_trait.push_str(&token.to_string());
            }
        }
    }

    // Don't forget the last trait
    if !current_trait.is_empty() {
        if let Ok(path) = syn::parse_str::<syn::Path>(current_trait.trim()) {
            traits.push(path);
        }
    }

    traits
}

fn impl_type_macro_derive_tricks(derive_traits: &[syn::Path], input: &DeriveInput) -> TokenStream2 {
    let mut macro_types = HashMap::new();
    let mut type_aliases = Vec::new();

    // Step 1: Collect all macro types and generate aliases
    collect_macro_types(&input.data, &input.generics, &mut macro_types);

    // Step 2: Generate type aliases
    for (macro_type, alias_name) in &macro_types {
        // Generate type aliases with only the specific generic parameters used by the macro
        // and add #[doc(hidden)] to hide them from documentation
        let used_generic_params = get_used_generic_params(macro_type, &input.generics);

        let alias = if used_generic_params.is_empty() {
            quote! {
                #[doc(hidden)]
                type #alias_name = #macro_type;
            }
        } else {
            // Create a filtered Generics struct with only the used parameters
            let filtered_generics = create_filtered_generics(&used_generic_params)
                .params
                .into_iter()
                .map(|mut param| {
                    match &mut param {
                        syn::GenericParam::Type(tp) => {
                            tp.eq_token = None;
                            tp.default = None;
                        }
                        syn::GenericParam::Const(cp) => {
                            cp.eq_token = None;
                            cp.default = None;
                        }
                        _ => (),
                    }
                    param
                })
                .collect::<Punctuated<_, syn::Token![,]>>();
            quote! {
                #[doc(hidden)]
                type #alias_name <#filtered_generics> = #macro_type;
            }
        };
        type_aliases.push(alias);
    }

    // Step 3: Transform the original type by replacing macro types with aliases
    let transformed_input = transform_input(input, &macro_types);

    // Step 4: Generate derive attribute
    let derive_attrs = if !derive_traits.is_empty() {
        let traits: Vec<_> = derive_traits.iter().collect();
        quote! {
            #[derive(#(#traits),*)]
        }
    } else {
        quote! {}
    };

    // Step 5: Combine everything
    quote! {
        #(#type_aliases)*

        #derive_attrs
        #transformed_input
    }
}

fn collect_macro_types(data: &Data, generics: &Generics, macro_types: &mut HashMap<Type, Ident>) {
    match data {
        Data::Struct(data_struct) => {
            collect_macro_types_from_fields(&data_struct.fields, generics, macro_types);
        }
        Data::Enum(data_enum) => {
            for variant in &data_enum.variants {
                collect_macro_types_from_fields(&variant.fields, generics, macro_types);
            }
        }
        Data::Union(data_union) => {
            collect_macro_types_from_fields(
                &Fields::Named(data_union.fields.clone()),
                generics,
                macro_types,
            );
        }
    }
}

fn collect_macro_types_from_fields(
    fields: &Fields,
    generics: &Generics,
    macro_types: &mut HashMap<Type, Ident>,
) {
    match fields {
        Fields::Named(fields) => {
            for field in &fields.named {
                collect_macro_types_from_type(&field.ty, generics, macro_types);
            }
        }
        Fields::Unnamed(fields) => {
            for field in &fields.unnamed {
                collect_macro_types_from_type(&field.ty, generics, macro_types);
            }
        }
        Fields::Unit => {}
    }
}

fn collect_macro_types_from_type(
    ty: &Type,
    _generics: &Generics,
    macro_types: &mut HashMap<Type, Ident>,
) {
    // Handle macro types directly - create aliases only for actual macro invocations
    if let Type::Macro(_) = ty {
        if !macro_types.contains_key(ty) {
            let alias_name = generate_random_type_name();
            macro_types.insert(ty.clone(), alias_name);
        }
        return;
    }

    // Recursively check all nested types for macro invocations
    match ty {
        Type::Path(type_path) => {
            for segment in &type_path.path.segments {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in &args.args {
                        if let syn::GenericArgument::Type(nested_ty) = arg {
                            collect_macro_types_from_type(nested_ty, _generics, macro_types);
                        }
                    }
                }
            }
        }
        Type::Array(type_array) => {
            collect_macro_types_from_type(&type_array.elem, _generics, macro_types);
        }
        Type::Ptr(type_ptr) => {
            collect_macro_types_from_type(&type_ptr.elem, _generics, macro_types);
        }
        Type::Reference(type_ref) => {
            collect_macro_types_from_type(&type_ref.elem, _generics, macro_types);
        }
        Type::Slice(type_slice) => {
            collect_macro_types_from_type(&type_slice.elem, _generics, macro_types);
        }
        Type::Tuple(type_tuple) => {
            for elem in &type_tuple.elems {
                collect_macro_types_from_type(elem, _generics, macro_types);
            }
        }
        _ => {}
    }
}

fn generate_random_type_name() -> Ident {
    let random_suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    Ident::new(
        &format!("__TypeMacroAlias{}", random_suffix),
        proc_macro2::Span::call_site(),
    )
}

fn get_used_generic_params(macro_type: &Type, generics: &Generics) -> Vec<syn::GenericParam> {
    // Analyze which specific generic parameters are used in the macro type
    let mut used_params = Vec::new();

    if let Type::Macro(type_macro) = macro_type {
        let macro_tokens = &type_macro.mac.tokens;

        for param in &generics.params {
            let param_name = match param {
                syn::GenericParam::Type(type_param) => type_param.ident.to_string(),
                syn::GenericParam::Lifetime(lifetime_param) => lifetime_param.lifetime.to_string(),
                syn::GenericParam::Const(const_param) => const_param.ident.to_string(),
            };

            // Use the improved token search that handles nested structures
            if is_generic_param_used_in_token_stream(macro_tokens, &param_name) {
                used_params.push(param.clone());
            }
        }
    }

    used_params
}

fn is_generic_param_used_in_token_stream(
    tokens: &proc_macro2::TokenStream,
    identifier: &str,
) -> bool {
    use proc_macro2::TokenTree;

    let tokens_vec: Vec<TokenTree> = tokens.clone().into_iter().collect();

    for (i, token) in tokens_vec.iter().enumerate() {
        match token {
            TokenTree::Ident(ident) => {
                // Handle regular type parameters and const parameters
                if *ident == identifier {
                    return true;
                }
            }
            TokenTree::Group(group) => {
                // Recursively search inside groups (brackets, braces, parentheses)
                if is_generic_param_used_in_token_stream(&group.stream(), identifier) {
                    return true;
                }
            }
            TokenTree::Punct(punct) => {
                // Handle lifetimes: look for ' followed by an identifier
                if punct.as_char() == '\'' && i + 1 < tokens_vec.len() {
                    if let TokenTree::Ident(ident) = &tokens_vec[i + 1] {
                        let lifetime = format!("'{}", ident);
                        if lifetime == identifier {
                            return true;
                        }
                    }
                }
            }
            TokenTree::Literal(_) => {
                // Literals don't contain type parameters
                continue;
            }
        }
    }

    false
}

fn create_filtered_generics(used_params: &[syn::GenericParam]) -> syn::Generics {
    // Create a new Generics struct containing only the used parameters
    let mut generics = syn::Generics::default();

    for param in used_params {
        generics.params.push(param.clone());
    }

    generics
}

fn transform_input(input: &DeriveInput, macro_types: &HashMap<Type, Ident>) -> DeriveInput {
    let mut transformed = input.clone();

    match &mut transformed.data {
        Data::Struct(data_struct) => {
            transform_fields(&mut data_struct.fields, macro_types, &input.generics);
        }
        Data::Enum(data_enum) => {
            for variant in &mut data_enum.variants {
                transform_fields(&mut variant.fields, macro_types, &input.generics);
            }
        }
        Data::Union(data_union) => {
            let mut fields = Fields::Named(data_union.fields.clone());
            transform_fields(&mut fields, macro_types, &input.generics);
            if let Fields::Named(named_fields) = fields {
                data_union.fields = named_fields;
            }
        }
    }

    transformed
}

fn transform_fields(fields: &mut Fields, macro_types: &HashMap<Type, Ident>, generics: &Generics) {
    match fields {
        Fields::Named(fields) => {
            for field in &mut fields.named {
                transform_type(&mut field.ty, macro_types, generics);
            }
        }
        Fields::Unnamed(fields) => {
            for field in &mut fields.unnamed {
                transform_type(&mut field.ty, macro_types, generics);
            }
        }
        Fields::Unit => {}
    }
}

fn transform_type(ty: &mut Type, macro_types: &HashMap<Type, Ident>, generics: &Generics) {
    // Handle macro types directly
    if let Type::Macro(_) = ty {
        // Check if this macro type has an alias
        if let Some(alias) = macro_types.get(ty) {
            let used_generic_params = get_used_generic_params(ty, generics);

            if used_generic_params.is_empty() {
                *ty = syn::parse_quote!(#alias);
            } else {
                // Create filtered generics and use them
                let filtered_generics = create_filtered_generics(&used_generic_params);
                let (_, ty_generics, _) = filtered_generics.split_for_impl();
                *ty = syn::parse_quote!(#alias #ty_generics);
            }
        }
        return;
    }

    // Recursively transform nested types, looking for macro parts within them
    match ty {
        Type::Path(type_path) => {
            for segment in &mut type_path.path.segments {
                if let syn::PathArguments::AngleBracketed(args) = &mut segment.arguments {
                    for arg in &mut args.args {
                        if let syn::GenericArgument::Type(nested_ty) = arg {
                            transform_type(nested_ty, macro_types, generics);
                        }
                    }
                }
            }
        }
        Type::Array(type_array) => {
            transform_type(&mut type_array.elem, macro_types, generics);
        }
        Type::Ptr(type_ptr) => {
            transform_type(&mut type_ptr.elem, macro_types, generics);
        }
        Type::Reference(type_ref) => {
            transform_type(&mut type_ref.elem, macro_types, generics);
        }
        Type::Slice(type_slice) => {
            transform_type(&mut type_slice.elem, macro_types, generics);
        }
        Type::Tuple(type_tuple) => {
            for elem in &mut type_tuple.elems {
                transform_type(elem, macro_types, generics);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_type_name() {
        let name1 = generate_random_type_name();
        let name2 = generate_random_type_name();

        assert_ne!(name1, name2);
        assert!(name1.to_string().starts_with("__TypeMacroAlias"));
        assert!(name2.to_string().starts_with("__TypeMacroAlias"));
    }
}
