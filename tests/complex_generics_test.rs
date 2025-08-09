use std::collections::HashMap;
use std::marker::PhantomData;
use type_macro_derive_tricks::macro_derive;

// Type-position macros that can work with type parameters
macro_rules! TypeMap {
    ($k:ty, $v:ty) => { HashMap<$k, $v> };
}

macro_rules! TypePair {
    ($t:ty, $u:ty) => {
        ($t, $u)
    };
}

macro_rules! TypeBox {
    ($t:ty) => { Box<$t> };
}

macro_rules! TypeVec {
    ($t:ty) => { Vec<$t> };
}

macro_rules! TypeResult {
    ($t:ty, $e:ty) => { Result<$t, $e> };
}

macro_rules! TypeWithPhantom {
    ($t:ty, $marker:ty) => { ($t, PhantomData<$marker>) };
}

// Test struct with multiple complex generic parameters
#[macro_derive(Debug, Clone, PartialEq)]
pub struct ComplexGenericStruct<T, U, V, W>
where
    T: Clone + std::fmt::Debug,
    U: std::fmt::Display,
    V: Default,
    W: Send + Sync,
{
    pub data_map: TypeMap![String, T],
    pub pair_data: TypePair![U, V],
    pub boxed_data: TypeBox![W],
    pub vec_data: TypeVec![T],
    pub result_data: TypeResult![U, String],
    pub phantom_data: TypeWithPhantom![T, (U, V, W)],
}

// Test enum with lifetime parameters and complex bounds
#[macro_derive(Debug, Clone)]
pub enum ComplexGenericEnum<'a, T, U>
where
    T: Clone + 'a,
    U: std::fmt::Debug + Send,
{
    Simple(TypeBox![T]),
    WithLifetime {
        reference: &'a T,
        data: TypePair![T, U],
        collection: TypeVec![U],
    },
    Complex(TypeMap![&'a str, TypeResult![T, U]]),
}

// Test struct with const generics and type macros
#[macro_derive(Debug, Clone, PartialEq)]
pub struct GenericArray<T, const N: usize>
where
    T: Clone + std::fmt::Debug,
{
    pub fixed_array: [T; N],
    pub dynamic_vec: TypeVec![T],
    pub boxed_data: TypeBox![T],
    pub result_array: TypeResult![[T; N], String],
}

// Test struct with higher-kinded types and complex nesting
#[macro_derive(Debug, Clone)]
pub struct HigherKindedStruct<T, F, G>
where
    T: Clone,
    F: Fn(T) -> T,
    G: FnOnce(T) -> Result<T, String>,
{
    pub data: T,
    pub transformer: TypeBox![F],
    pub processor: TypeBox![G],
    pub results: TypeVec![TypeResult![T, String]],
    pub metadata: TypeMap![String, TypeWithPhantom![T, (F, G)]],
}

// Test enum with associated types (simplified)
pub trait ComplexTrait {
    type Item;
    type Error;
}

impl ComplexTrait for String {
    type Item = i32;
    type Error = &'static str;
}

#[macro_derive(Debug, Clone)]
pub enum TraitBasedEnum<T, U>
where
    T: ComplexTrait + Clone + std::fmt::Debug,
    U: std::fmt::Display + Clone + std::fmt::Debug,
{
    TraitData(TypePair![i32, T]), // Concrete types instead of associated types
    Combined {
        trait_item: TypeBox![i32],
        display_item: TypeBox![U],
        collection: TypeVec![i32],
    },
    Nested(TypeMap![String, TypeWithPhantom![i32, U]]),
}

// Test struct with recursive-like generic structure
#[macro_derive(Debug, Clone)]
pub struct RecursiveLike<T>
where
    T: Clone + std::fmt::Debug,
{
    pub data: T,
    pub nested: TypeBox![TypeVec![T]],
    pub deep_nested: TypeBox![TypeMap![String, TypeVec![T]]],
    pub paired: TypePair![T, TypeBox![T]],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct SimpleItem(i32);

    impl ComplexTrait for SimpleItem {
        type Item = i32;
        type Error = &'static str;
    }

    impl std::fmt::Display for SimpleItem {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SimpleItem({})", self.0)
        }
    }

    #[test]
    fn test_complex_generic_struct() {
        let mut data_map = HashMap::new();
        data_map.insert("key".to_string(), 42i32);

        let instance = ComplexGenericStruct::<i32, String, bool, f64> {
            data_map,
            pair_data: ("hello".to_string(), true),
            boxed_data: Box::new(3.15),
            vec_data: vec![1, 2, 3],
            result_data: Ok("success".to_string()),
            phantom_data: (100, PhantomData),
        };

        // Test Debug trait
        let debug_str = format!("{:?}", instance);
        assert!(debug_str.contains("ComplexGenericStruct"));

        // Test Clone trait
        let cloned = instance.clone();
        assert_eq!(cloned.vec_data.len(), 3);
        assert!(cloned.pair_data.1);

        // Test type correctness
        assert_eq!(*cloned.boxed_data, 3.15f64);
        assert_eq!(cloned.data_map.get("key"), Some(&42i32));
    }

    #[test]
    fn test_complex_generic_enum() {
        let test_data = 42i32;
        let test_ref = &test_data;

        let variants = vec![
            ComplexGenericEnum::<i32, String>::Simple(Box::new(100)),
            ComplexGenericEnum::WithLifetime {
                reference: test_ref,
                data: (200, "test".to_string()),
                collection: vec!["a".to_string(), "b".to_string()],
            },
        ];

        // Test Debug trait
        let debug_str = format!("{:?}", variants[0]);
        assert!(debug_str.contains("Simple"));

        // Test Clone trait
        let cloned = variants.clone();
        assert_eq!(cloned.len(), 2);

        // Test pattern matching
        match &variants[1] {
            ComplexGenericEnum::WithLifetime {
                reference,
                data,
                collection,
            } => {
                assert_eq!(**reference, 42);
                assert_eq!(data.0, 200);
                assert_eq!(collection.len(), 2);
            }
            _ => panic!("Expected WithLifetime variant"),
        }
    }

    #[test]
    fn test_generic_array() {
        let instance = GenericArray::<String, 3> {
            fixed_array: ["a".to_string(), "b".to_string(), "c".to_string()],
            dynamic_vec: vec!["x".to_string(), "y".to_string()],
            boxed_data: Box::new("boxed".to_string()),
            result_array: Ok(["1".to_string(), "2".to_string(), "3".to_string()]),
        };

        // Test Debug trait
        let debug_str = format!("{:?}", instance);
        assert!(debug_str.contains("GenericArray"));

        // Test Clone trait
        let cloned = instance.clone();
        assert_eq!(cloned.fixed_array.len(), 3);
        assert_eq!(cloned.dynamic_vec.len(), 2);

        // Test const generic correctness
        assert_eq!(cloned.fixed_array[0], "a");
        assert_eq!(*cloned.boxed_data, "boxed");
    }

    #[test]
    fn test_higher_kinded_struct() {
        let add_one = |x: i32| x + 1;
        let process = |x: i32| Ok(x * 2);

        let instance = HigherKindedStruct {
            data: 10,
            transformer: Box::new(add_one),
            processor: Box::new(process),
            results: vec![Ok(1), Err("error".to_string()), Ok(3)],
            metadata: {
                let mut map = HashMap::new();
                map.insert("info".to_string(), (42, PhantomData));
                map
            },
        };

        // Test Debug trait (functions don't implement Debug, so this tests compilation)
        // We can't easily test Debug for function types, but we can test the structure
        assert_eq!(instance.data, 10);
        assert_eq!(instance.results.len(), 3);
        assert_eq!(instance.metadata.len(), 1);

        // Test Clone trait
        let cloned = instance.clone();
        assert_eq!(cloned.data, 10);
        assert_eq!(cloned.results.len(), 3);
    }

    #[test]
    fn test_trait_based_enum() {
        let variants = vec![
            TraitBasedEnum::<SimpleItem, String>::TraitData((42, SimpleItem(99))),
            TraitBasedEnum::Combined {
                trait_item: Box::new(100),
                display_item: Box::new("display_value".to_string()),
                collection: vec![1, 2, 3],
            },
        ];

        // Test Debug trait
        let debug_str = format!("{:?}", variants[0]);
        assert!(debug_str.contains("TraitData"));

        // Test Clone trait
        let cloned = variants.clone();
        assert_eq!(cloned.len(), 2);

        // Test pattern matching
        match &variants[1] {
            TraitBasedEnum::Combined {
                trait_item,
                display_item,
                collection,
            } => {
                assert_eq!(**trait_item, 100);
                assert_eq!(**display_item, "display_value");
                assert_eq!(collection.len(), 3);
            }
            _ => panic!("Expected Combined variant"),
        }
    }

    #[test]
    fn test_recursive_like() {
        let instance = RecursiveLike {
            data: "root".to_string(),
            nested: Box::new(vec!["a".to_string(), "b".to_string()]),
            deep_nested: Box::new({
                let mut map = HashMap::new();
                map.insert("key".to_string(), vec!["x".to_string(), "y".to_string()]);
                map
            }),
            paired: ("left".to_string(), Box::new("right".to_string())),
        };

        // Test Debug trait
        let debug_str = format!("{:?}", instance);
        assert!(debug_str.contains("RecursiveLike"));

        // Test Clone trait
        let cloned = instance.clone();
        assert_eq!(cloned.data, "root");
        assert_eq!(cloned.nested.len(), 2);
        assert_eq!(cloned.deep_nested.len(), 1);

        // Test nested structure access
        assert_eq!((*cloned.nested)[0], "a");
        assert_eq!(cloned.paired.0, "left");
        assert_eq!(*cloned.paired.1, "right");
    }

    #[test]
    fn test_macro_expansion_with_generics() {
        // This test verifies that type macros are correctly expanded
        // while preserving generic parameters

        // Create type-checked instances to verify compilation
        let map: HashMap<String, i32> = HashMap::new();
        let pair: (String, bool) = ("test".to_string(), true);
        let boxed: Box<f64> = Box::new(3.15);
        let vec_data: Vec<i32> = vec![1, 2, 3];
        let result: Result<String, String> = Ok("success".to_string());
        let phantom: (i32, PhantomData<(String, bool, f64)>) = (42, PhantomData);

        // Verify types are correctly resolved
        assert_eq!(map.len(), 0);
        assert_eq!(pair.0, "test");
        assert_eq!(*boxed, 3.15);
        assert_eq!(vec_data.len(), 3);
        assert!(result.is_ok());
        assert_eq!(phantom.0, 42);

        // Test that generic parameters are preserved through macro expansion
        let complex_instance = ComplexGenericStruct::<i32, String, bool, f64> {
            data_map: map,
            pair_data: pair,
            boxed_data: boxed,
            vec_data,
            result_data: result,
            phantom_data: phantom,
        };

        // Verify generic constraints are maintained
        let cloned = complex_instance.clone();
        let debug_output = format!("{:?}", cloned);
        assert!(!debug_output.is_empty()); // Debug trait works
        assert_eq!(cloned, complex_instance); // PartialEq trait works
    }
}
