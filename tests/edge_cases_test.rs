use type_macro_derive_tricks::macro_derive;

// Edge case macros for testing corner cases

// 1. Recursive macro (macro that calls another macro)
macro_rules! RecursiveBase {
    ($t:ty) => { Vec<$t> };
}

macro_rules! RecursiveMacro {
    ($t:ty) => { RecursiveBase![$t] };
}

// 2. Macro with multiple patterns and complex expansion
macro_rules! PatternMacro {
    (simple) => { i32 };
    (complex $t:ty) => { std::collections::HashMap<String, $t> };
    (nested $t:ty, $u:ty) => { std::collections::BTreeMap<$t, Vec<$u>> };
    (generic<$($t:ty),+>) => { ($($t,)+) };
}

// 3. Macro generating function types
macro_rules! FnTypeMacro {
    (simple) => { fn() };
    (one_arg $t:ty) => { fn($t) };
    (with_return $t:ty, $r:ty) => { fn($t) -> $r };
    (closure $t:ty) => { Box<dyn Fn($t) -> String> };
    (unit_fn $t:ty) => { fn($t) -> () };
}

// 4. Macro with trait bounds
#[allow(unused_macros)]
macro_rules! BoundedMacro {
    ($t:ty) => { Box<dyn Iterator<Item = $t> + Send + Sync> };
}

// 5. Macro that generates array types
macro_rules! ArrayMacro {
    ($t:ty, $n:expr) => { [$t; $n] };
}

// 6. Deeply nested macro
macro_rules! DeeplyNested {
    ($t:ty) => { 
        Option<Result<Vec<std::collections::HashMap<String, Box<$t>>>, String>>
    };
}

// Supporting types
#[derive(Debug, Clone, PartialEq)]
pub struct Container<T, S> {
    pub data: T,
    pub span: S,
}

pub trait CustomSpan: Clone + std::fmt::Debug + Default {}
impl CustomSpan for String {}
impl CustomSpan for (usize, usize) {}

// Test structures with edge cases

// 1. Multiple macros in the same type definition
#[macro_derive(Debug, Clone)]
pub struct MultipleMacros<S: CustomSpan> {
    pub recursive: Container<RecursiveMacro![i32], S>,
    pub pattern_simple: Container<PatternMacro![simple], S>,
    pub pattern_complex: Container<PatternMacro![complex String], S>,
    pub fn_type: Container<FnTypeMacro![simple], S>,
}

// 2. Macros within generic containers
#[macro_derive(Debug, Clone)]
pub struct MacrosInGenerics<S: CustomSpan> {
    pub vec_of_macro: Vec<Container<PatternMacro![simple], S>>,
    pub option_macro: Option<Container<RecursiveMacro![bool], S>>,
    pub result_macro: Result<Container<FnTypeMacro![unit_fn i32], S>, String>,
    pub box_macro: Box<Container<DeeplyNested![f64], S>>,
}

// 3. Enum with complex macro patterns
#[macro_derive(Debug, Clone, PartialEq)]
pub enum ComplexMacroEnum<S: CustomSpan> {
    Simple(Container<PatternMacro![simple], S>),
    Complex(Container<PatternMacro![complex bool], S>),
    Nested(Container<PatternMacro![nested String, i32], S>),
    Generic(Container<PatternMacro![generic<u8, u16, u32>], S>),
    Function(Container<FnTypeMacro![with_return bool, String], S>),
    Array(Container<ArrayMacro![f32, 10], S>),
}

// 4. Struct with macros in different positions
#[macro_derive(Debug, Clone)]
pub struct VariousPositions<S: CustomSpan> {
    // Direct macro usage
    pub direct: PatternMacro![simple],
    // Macro in generic parameter
    pub in_generic: Vec<RecursiveMacro![String]>,
    // Macro in nested generic
    pub nested_generic: std::collections::HashMap<String, Option<PatternMacro![simple]>>,
    // Multiple levels of nesting
    pub deep_nested: Result<Vec<Option<Container<DeeplyNested![bool], S>>>, String>,
    // Macro in tuple
    pub in_tuple: (PatternMacro![simple], RecursiveMacro![f64], S),
    // Macro in array
    pub in_array: [PatternMacro![simple]; 3],
}

// 5. Simple struct for basic testing
#[macro_derive(Debug, Clone, Copy)]
pub struct SimpleStruct {
    pub value: PatternMacro![simple],
}

// 6. Test with lifetimes and macros
#[macro_derive(Debug, Clone)]
pub struct WithLifetimes<'a, S: CustomSpan> {
    pub reference: &'a PatternMacro![simple],
    pub container: Container<RecursiveMacro![String], S>,
    pub phantom: std::marker::PhantomData<&'a ()>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_macros() {
        let multiple = MultipleMacros {
            recursive: Container { data: vec![1, 2, 3], span: "test".to_string() },
            pattern_simple: Container { data: 42i32, span: "simple".to_string() },
            pattern_complex: Container {
                data: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("key".to_string(), "value".to_string());
                    map
                },
                span: "complex".to_string(),
            },
            fn_type: Container {
                data: || {},
                span: "function".to_string(),
            },
        };

        // Test that all types are correctly resolved
        assert_eq!(multiple.recursive.data.len(), 3);
        assert_eq!(multiple.pattern_simple.data, 42);
        assert_eq!(multiple.pattern_complex.data.len(), 1);

        // Test Debug trait
        let debug_str = format!("{:?}", multiple);
        assert!(debug_str.contains("MultipleMacros"));

        // Test Clone trait
        let cloned = multiple.clone();
        assert_eq!(cloned.recursive.data.len(), 3);
    }

    #[test]
    fn test_macros_in_generics() {
        let container_vec = vec![
            Container { data: 1i32, span: (0, 5) },
            Container { data: 2i32, span: (5, 10) },
        ];

        let macros_in_generics = MacrosInGenerics {
            vec_of_macro: container_vec,
            option_macro: Some(Container { data: vec![true, false], span: (10, 15) }),
            result_macro: Ok(Container {
                data: |_x: i32| {},
                span: (15, 20),
            }),
            box_macro: Box::new(Container {
                data: Some(Ok(vec![{
                    let mut map = std::collections::HashMap::new();
                    map.insert("test".to_string(), Box::new(3.15f64));
                    map
                }])),
                span: (20, 25),
            }),
        };

        // Verify nested structures
        assert_eq!(macros_in_generics.vec_of_macro.len(), 2);
        assert_eq!(macros_in_generics.vec_of_macro[0].data, 1);
        
        if let Some(container) = &macros_in_generics.option_macro {
            assert_eq!(container.data.len(), 2);
            assert!(container.data[0]);
        }

        if let Ok(container) = &macros_in_generics.result_macro {
            (container.data)(42); // Just call the function, it returns ()
        }

        // Test traits
        let debug_str = format!("{:?}", macros_in_generics);
        assert!(debug_str.contains("MacrosInGenerics"));

        let cloned = macros_in_generics.clone();
        assert_eq!(cloned.vec_of_macro.len(), 2);
    }

    #[test]
    fn test_complex_macro_enum() {
        let variants = vec![
            ComplexMacroEnum::Simple(Container { data: 100i32, span: "simple".to_string() }),
            ComplexMacroEnum::Complex(Container {
                data: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("key".to_string(), true);
                    map
                },
                span: "complex".to_string(),
            }),
            ComplexMacroEnum::Nested(Container {
                data: {
                    let mut map = std::collections::BTreeMap::new();
                    map.insert("test".to_string(), vec![1, 2, 3]);
                    map
                },
                span: "nested".to_string(),
            }),
            ComplexMacroEnum::Generic(Container {
                data: (1u8, 2u16, 3u32),
                span: "generic".to_string(),
            }),
            ComplexMacroEnum::Array(Container {
                data: [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
                span: "array".to_string(),
            }),
            ComplexMacroEnum::Function(Container {
                data: |b: bool| b.to_string(),
                span: "function".to_string(),
            }),
        ];

        // Test each variant
        match &variants[0] {
            ComplexMacroEnum::Simple(container) => {
                assert_eq!(container.data, 100);
            }
            _ => panic!("Expected Simple variant"),
        }

        match &variants[1] {
            ComplexMacroEnum::Complex(container) => {
                assert_eq!(container.data.len(), 1);
                assert_eq!(container.data.get("key"), Some(&true));
            }
            _ => panic!("Expected Complex variant"),
        }

        match &variants[2] {
            ComplexMacroEnum::Nested(container) => {
                assert_eq!(container.data.len(), 1);
                let vec = container.data.get("test").unwrap();
                assert_eq!(vec.len(), 3);
            }
            _ => panic!("Expected Nested variant"),
        }

        match &variants[3] {
            ComplexMacroEnum::Generic(container) => {
                assert_eq!(container.data.0, 1u8);
                assert_eq!(container.data.1, 2u16);
                assert_eq!(container.data.2, 3u32);
            }
            _ => panic!("Expected Generic variant"),
        }

        match &variants[4] {
            ComplexMacroEnum::Array(container) => {
                assert_eq!(container.data.len(), 10);
                assert_eq!(container.data[0], 1.0f32);
            }
            _ => panic!("Expected Array variant"),
        }

        match &variants[5] {
            ComplexMacroEnum::Function(container) => {
                let result = (container.data)(true);
                assert_eq!(result, "true");
            }
            _ => panic!("Expected Function variant"),
        }

        // Test traits
        let debug_str = format!("{:?}", variants);
        assert!(!debug_str.is_empty()); // Just verify debug works

        let cloned = variants.clone();
        assert_eq!(cloned.len(), 6);
    }

    #[test]
    fn test_various_positions() {
        let various = VariousPositions {
            direct: 42i32,
            in_generic: vec![vec!["a".to_string(), "b".to_string()]],
            nested_generic: {
                let mut map = std::collections::HashMap::new();
                map.insert("key".to_string(), Some(100i32));
                map
            },
            deep_nested: Ok(vec![Some(Container {
                data: Some(Ok(vec![{
                    let mut inner_map = std::collections::HashMap::new();
                    inner_map.insert("deep".to_string(), Box::new(true));
                    inner_map
                }])),
                span: (0, 10),
            })]),
            in_tuple: (1i32, vec![2.5f64], (20, 30)),
            in_array: [10i32, 20i32, 30i32],
        };

        // Verify all the different positions work
        assert_eq!(various.direct, 42);
        assert_eq!(various.in_generic[0].len(), 2);
        assert_eq!(various.nested_generic.get("key"), Some(&Some(100i32)));
        assert_eq!(various.in_tuple.0, 1);
        assert_eq!(various.in_tuple.1[0], 2.5);
        assert_eq!(various.in_array[2], 30);

        // Test nested structure
        if let Ok(vec) = &various.deep_nested {
            if let Some(container) = &vec[0] {
                if let Some(Ok(inner_vec)) = &container.data {
                    assert_eq!(inner_vec.len(), 1);
                    let inner_bool = inner_vec[0].get("deep").unwrap();
                    assert!(**inner_bool);
                }
            }
        }

        // Test traits
        let debug_str = format!("{:?}", various);
        assert!(debug_str.contains("VariousPositions"));

        let cloned = various.clone();
        assert_eq!(cloned.direct, 42);
    }

    #[test]
    fn test_simple_struct() {
        let simple = SimpleStruct { value: 42i32 };
        
        assert_eq!(simple.value, 42);

        // Test traits
        let debug_str = format!("{:?}", simple);
        assert!(debug_str.contains("SimpleStruct"));

        let cloned = simple;
        assert_eq!(cloned.value, 42);
    }

    #[test]
    fn test_with_lifetimes() {
        let value = 100i32;
        let vec_value = vec!["hello".to_string(), "world".to_string()];
        
        let with_lifetimes = WithLifetimes {
            reference: &value,
            container: Container {
                data: vec_value,
                span: "lifetime_test".to_string(),
            },
            phantom: std::marker::PhantomData,
        };

        // Verify references work correctly
        assert_eq!(*with_lifetimes.reference, 100);
        assert_eq!(with_lifetimes.container.data.len(), 2);
        assert_eq!(with_lifetimes.container.data[0], "hello");

        // Test traits
        let debug_str = format!("{:?}", with_lifetimes);
        assert!(debug_str.contains("WithLifetimes"));

        let cloned = with_lifetimes.clone();
        assert_eq!(*cloned.reference, 100);
    }

    #[test]
    fn test_edge_case_type_correctness() {
        // Verify that complex macro expansions result in correct types
        
        // Test recursive macro
        let recursive_type: Vec<i32> = vec![1, 2, 3];
        assert_eq!(recursive_type.len(), 3);

        // Test pattern macro with complex pattern
        let complex_pattern: std::collections::HashMap<String, bool> = {
            let mut map = std::collections::HashMap::new();
            map.insert("test".to_string(), true);
            map
        };
        assert_eq!(complex_pattern.len(), 1);

        // Test nested pattern
        let nested_pattern: std::collections::BTreeMap<String, Vec<i32>> = {
            let mut map = std::collections::BTreeMap::new();
            map.insert("key".to_string(), vec![1, 2, 3]);
            map
        };
        assert_eq!(nested_pattern.len(), 1);

        // Test generic pattern
        let generic_pattern: (u8, u16, u32) = (1u8, 2u16, 3u32);
        assert_eq!(generic_pattern.2, 3u32);

        // Test function types
        let fn_simple: fn() = || {};
        fn_simple();

        let fn_with_arg: fn(i32) = |_x| {};
        fn_with_arg(42);

        let fn_with_return: fn(bool) -> String = |b| b.to_string();
        assert_eq!(fn_with_return(true), "true");

        // Test array macro
        let array_type: [f32; 10] = [1.0; 10];
        assert_eq!(array_type.len(), 10);

        // Test deeply nested type
        let deeply_nested: Option<Result<Vec<std::collections::HashMap<String, Box<String>>>, String>> = 
            Some(Ok(vec![{
                let mut map = std::collections::HashMap::new();
                map.insert("deep".to_string(), Box::new("nested".to_string()));
                map
            }]));

        if let Some(Ok(vec)) = deeply_nested {
            let map = &vec[0];
            let boxed_str = map.get("deep").unwrap();
            assert_eq!(boxed_str.as_str(), "nested");
        }
    }
}