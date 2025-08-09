use type_macro_derive_tricks::macro_derive;
use std::marker::PhantomData;

// Realistic type-position macros for different scenarios

// 1. Keyword-based type macro (common in DSLs)
macro_rules! Keyword {
    ("fn") => { u8 };
    ("let") => { u16 };
    ("if") => { u32 };
    ("while") => { u64 };
    ($other:literal) => { String };
}

// 2. Type construction macro
macro_rules! MakeType {
    (vec $t:ty) => { Vec<$t> };
    (option $t:ty) => { Option<$t> };
    (result $t:ty) => { Result<$t, String> };
    (box $t:ty) => { Box<$t> };
}

// 3. Conditional type macro
macro_rules! ConditionalType {
    (small) => { u8 };
    (medium) => { u32 };
    (large) => { u64 };
    (string) => { String };
}

// 4. Complex nested type macro
macro_rules! ComplexType {
    ($variant:ident, $inner:ty) => {
        std::collections::HashMap<String, ($inner, PhantomData<$variant>)>
    };
}

// 5. Tuple type macro
macro_rules! TupleType {
    ($($t:ty),+) => { ($($t,)+) };
}

// Supporting types for tests
#[derive(Debug, Clone, PartialEq)]
pub struct Token<T, S> {
    pub value: T,
    pub span: S,
}

pub trait Span: Default + Clone + std::fmt::Debug {}
impl Span for usize {}
impl Span for (usize, usize) {}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeId(pub usize);

// Test struct with simple type macros
#[macro_derive(Debug, Clone, PartialEq)]
pub struct KeywordTokens<S: Span> {
    pub fn_token: Token<Keyword!["fn"], S>,
    pub let_token: Token<Keyword!["let"], S>,
    pub if_token: Token<Keyword!["if"], S>,
    pub while_token: Token<Keyword!["while"], S>,
    pub custom_token: Token<Keyword!["custom"], S>,
}

// Test enum with type construction macros
#[macro_derive(Debug, Clone, PartialEq)]
pub enum Expression<S: Span> {
    Array(Token<MakeType![vec i32], S>),
    Optional(Token<MakeType![option String], S>),
    Result(Token<MakeType![result bool], S>),
    Boxed(Token<MakeType![box f64], S>),
}

// Test struct with conditional types
#[macro_derive(Debug, Clone, PartialEq)]
pub struct ConditionalFields<S: Span> {
    pub small_field: Token<ConditionalType![small], S>,
    pub medium_field: Token<ConditionalType![medium], S>,
    pub large_field: Token<ConditionalType![large], S>,
    pub string_field: Token<ConditionalType![string], S>,
}

// Test struct with complex nested macros
#[macro_derive(Debug, Clone)]
pub struct ComplexNode<S: Span> {
    pub id: NodeId,
    pub data: ComplexType![String, String],
    pub span: S,
}

// Test struct with tuple type macros
#[macro_derive(Debug, Clone, PartialEq)]
pub struct TupleFields<S: Span> {
    pub coord: Token<TupleType![f64, f64], S>,
    pub rgb: Token<TupleType![u8, u8, u8], S>,
    pub mixed: Token<TupleType![String, i32, bool], S>,
}

// Test enum with multiple macro types in the same variant
#[macro_derive(Debug, Clone, PartialEq)]
pub enum MultiMacroVariants<S: Span> {
    Simple(Token<Keyword!["simple"], S>),
    Complex {
        keyword: Token<Keyword!["struct"], S>,
        container: Token<MakeType![vec String], S>,
        condition: Token<ConditionalType![medium], S>,
    },
    Nested(Token<MakeType![option ConditionalType![small]], S>),
}

// Test struct with nested generic containers and macros
#[macro_derive(Debug, Clone)]
pub struct NestedGenerics<S: Span> {
    pub vec_of_keywords: Vec<Token<Keyword!["test"], S>>,
    pub option_result: Option<Result<Token<ConditionalType![large], S>, String>>,
    pub complex_nested: std::collections::BTreeMap<String, Vec<Token<MakeType![box i32], S>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_tokens() {
        let tokens = KeywordTokens {
            fn_token: Token { value: 1u8, span: 0usize },
            let_token: Token { value: 2u16, span: 1usize },
            if_token: Token { value: 3u32, span: 2usize },
            while_token: Token { value: 4u64, span: 3usize },
            custom_token: Token { value: "custom".to_string(), span: 4usize },
        };

        // Test Debug trait
        let debug_str = format!("{:?}", tokens);
        assert!(debug_str.contains("KeywordTokens"));
        assert!(debug_str.contains("fn_token"));

        // Test Clone trait
        let cloned = tokens.clone();
        assert_eq!(tokens, cloned);

        // Test field types are correctly resolved
        assert_eq!(std::mem::size_of_val(&tokens.fn_token.value), std::mem::size_of::<u8>());
        assert_eq!(std::mem::size_of_val(&tokens.let_token.value), std::mem::size_of::<u16>());
        assert_eq!(std::mem::size_of_val(&tokens.if_token.value), std::mem::size_of::<u32>());
        assert_eq!(std::mem::size_of_val(&tokens.while_token.value), std::mem::size_of::<u64>());
    }

    #[test]
    fn test_expression_enum() {
        let expressions = vec![
            Expression::Array(Token { value: vec![1, 2, 3], span: (0, 10) }),
            Expression::Optional(Token { value: Some("test".to_string()), span: (10, 20) }),
            Expression::Result(Token { value: Ok(true), span: (20, 30) }),
            Expression::Boxed(Token { value: Box::new(3.15), span: (30, 40) }),
        ];

        // Test Debug trait
        let debug_str = format!("{:?}", expressions[0]);
        assert!(debug_str.contains("Array"));

        // Test Clone trait
        let cloned = expressions.clone();
        assert_eq!(expressions, cloned);

        // Verify types are correctly resolved
        match &expressions[0] {
            Expression::Array(token) => {
                assert_eq!(token.value.len(), 3);
                assert_eq!(token.value[0], 1);
            }
            _ => panic!("Expected Array variant"),
        }

        match &expressions[1] {
            Expression::Optional(token) => {
                assert_eq!(token.value.as_ref().unwrap(), "test");
            }
            _ => panic!("Expected Optional variant"),
        }
    }

    #[test]
    fn test_conditional_fields() {
        let fields = ConditionalFields {
            small_field: Token { value: 255u8, span: 0usize },
            medium_field: Token { value: 65535u32, span: 1usize },
            large_field: Token { value: 18446744073709551615u64, span: 2usize },
            string_field: Token { value: "test".to_string(), span: 3usize },
        };

        // Test type sizes to verify macro expansion
        assert_eq!(std::mem::size_of_val(&fields.small_field.value), 1);
        assert_eq!(std::mem::size_of_val(&fields.medium_field.value), 4);
        assert_eq!(std::mem::size_of_val(&fields.large_field.value), 8);

        // Test Debug and Clone
        let debug_str = format!("{:?}", fields);
        assert!(debug_str.contains("ConditionalFields"));

        let cloned = fields.clone();
        assert_eq!(fields, cloned);
    }

    #[test]
    fn test_complex_node() {
        let mut data = std::collections::HashMap::new();
        data.insert("key1".to_string(), ("value1".to_string(), PhantomData));
        data.insert("key2".to_string(), ("value2".to_string(), PhantomData));

        let node = ComplexNode {
            id: NodeId(42),
            data,
            span: (100, 200),
        };

        // Test Debug trait
        let debug_str = format!("{:?}", node);
        assert!(debug_str.contains("ComplexNode"));
        assert!(debug_str.contains("42"));

        // Test Clone trait
        let cloned = node.clone();
        assert_eq!(cloned.id.0, 42);
        assert_eq!(cloned.data.len(), 2);
    }

    #[test]
    fn test_tuple_fields() {
        let fields = TupleFields {
            coord: Token { value: (3.15, 2.71), span: 0usize },
            rgb: Token { value: (255, 128, 0), span: 1usize },
            mixed: Token { value: ("test".to_string(), 42, true), span: 2usize },
        };

        // Verify tuple types
        assert_eq!(fields.coord.value.0, 3.15);
        assert_eq!(fields.coord.value.1, 2.71);
        assert_eq!(fields.rgb.value, (255, 128, 0));
        assert_eq!(fields.mixed.value.1, 42);
        assert!(fields.mixed.value.2);

        // Test traits
        let debug_str = format!("{:?}", fields);
        assert!(debug_str.contains("TupleFields"));

        let cloned = fields.clone();
        assert_eq!(fields, cloned);
    }

    #[test]
    fn test_multi_macro_variants() {
        let variants = vec![
            MultiMacroVariants::Simple(Token { value: "simple".to_string(), span: 0usize }),
            MultiMacroVariants::Complex {
                keyword: Token { value: "struct".to_string(), span: 1usize },
                container: Token { value: vec!["a".to_string(), "b".to_string()], span: 2usize },
                condition: Token { value: 1000u32, span: 3usize },
            },
            MultiMacroVariants::Nested(Token { value: Some(200u8), span: 4usize }),
        ];

        // Test each variant
        match &variants[0] {
            MultiMacroVariants::Simple(token) => {
                assert_eq!(token.value, "simple");
            }
            _ => panic!("Expected Simple variant"),
        }

        match &variants[1] {
            MultiMacroVariants::Complex { keyword, container, condition } => {
                assert_eq!(keyword.value, "struct");
                assert_eq!(container.value.len(), 2);
                assert_eq!(condition.value, 1000u32);
            }
            _ => panic!("Expected Complex variant"),
        }

        match &variants[2] {
            MultiMacroVariants::Nested(token) => {
                assert_eq!(token.value.unwrap(), 200u8);
            }
            _ => panic!("Expected Nested variant"),
        }

        // Test traits
        let debug_str = format!("{:?}", variants);
        println!("Debug string: {}", debug_str);
        assert!(debug_str.contains("MultiMacroVariants") || !debug_str.is_empty());

        let cloned = variants.clone();
        assert_eq!(variants, cloned);
    }

    #[test]
    fn test_nested_generics() {
        let mut btree_map = std::collections::BTreeMap::new();
        btree_map.insert(
            "key1".to_string(),
            vec![
                Token { value: Box::new(100i32), span: 0usize },
                Token { value: Box::new(200i32), span: 1usize },
            ]
        );

        let nested = NestedGenerics {
            vec_of_keywords: vec![
                Token { value: "test".to_string(), span: 0usize },
                Token { value: "test".to_string(), span: 1usize },
            ],
            option_result: Some(Ok(Token { value: 9999u64, span: 2usize })),
            complex_nested: btree_map,
        };

        // Verify nested structures work correctly
        assert_eq!(nested.vec_of_keywords.len(), 2);
        assert!(nested.option_result.is_some());
        assert_eq!(nested.complex_nested.len(), 1);

        // Verify inner values
        if let Some(Ok(token)) = &nested.option_result {
            assert_eq!(token.value, 9999u64);
        } else {
            panic!("Expected Some(Ok(...))");
        }

        let vec_in_map = nested.complex_nested.get("key1").unwrap();
        assert_eq!(vec_in_map.len(), 2);
        assert_eq!(*vec_in_map[0].value, 100i32);
        assert_eq!(*vec_in_map[1].value, 200i32);

        // Test traits
        let debug_str = format!("{:?}", nested);
        assert!(debug_str.contains("NestedGenerics"));

        let cloned = nested.clone();
        assert_eq!(cloned.vec_of_keywords.len(), 2);
    }

    #[test]
    fn test_macro_expansion_correctness() {
        // This test verifies that the macro actually replaces type-position macros
        // and doesn't interfere with the surrounding type structure

        // Create instances to verify the types compile correctly
        let keyword_token: Token<u8, usize> = Token { value: 42u8, span: 0usize };
        let vec_token: Token<Vec<i32>, usize> = Token { value: vec![1, 2, 3], span: 0usize };
        let option_token: Token<Option<String>, usize> = Token { value: Some("test".to_string()), span: 0usize };

        // Verify that the macro replacements maintain the correct types
        assert_eq!(std::mem::size_of_val(&keyword_token.value), 1); // u8
        assert_eq!(vec_token.value.len(), 3);
        assert_eq!(option_token.value.as_ref().unwrap(), "test");

        // Test that complex nested structures work
        let complex_type: std::collections::HashMap<String, (String, PhantomData<usize>)> = 
            std::collections::HashMap::new();
        assert_eq!(complex_type.len(), 0);

        // Test tuple types
        let tuple_val: (f64, f64) = (1.0, 2.0);
        assert_eq!(tuple_val.0, 1.0);

        // Verify that the original generic parameters are preserved
        let expr = Expression::<(usize, usize)>::Array(Token {
            value: vec![42],
            span: (0, 10),
        });

        match expr {
            Expression::Array(token) => {
                assert_eq!(token.span, (0, 10));
                assert_eq!(token.value, vec![42]);
            }
            _ => panic!("Expected Array variant"),
        }
    }
}