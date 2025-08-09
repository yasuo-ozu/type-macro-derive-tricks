use type_macro_derive_tricks::macro_derive;

// Test macro that uses generic parameters
macro_rules! GenericMacro {
    (vec $t:ty) => { Vec<$t> };
    (option $t:ty) => { Option<$t> };
}

// Test struct where the macro uses the generic parameter
#[macro_derive(Debug, Clone)]
pub struct TestStructWithGenericInMacro<T>
where 
    T: Clone + std::fmt::Debug,
{
    pub vec_field: GenericMacro![vec T],
    pub option_field: GenericMacro![option T],
}

// Test enum where the macro uses the generic parameter  
#[macro_derive(Debug, Clone)]
pub enum TestEnumWithGenericInMacro<T>
where
    T: Clone + std::fmt::Debug,
{
    Vec(GenericMacro![vec T]),
    Option(GenericMacro![option T]),
}

// Test struct with macro that doesn't use generics but struct has generics
#[macro_derive(Debug, Clone)]
pub struct TestStructMixed<T>
where
    T: Clone + std::fmt::Debug,
{
    pub concrete_field: GenericMacro![vec String], // Macro doesn't use T
    pub generic_field: T, // Direct use of T
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_with_generic_in_macro() {
        let instance = TestStructWithGenericInMacro {
            vec_field: vec![1, 2, 3],
            option_field: Some(42),
        };

        // Test Debug trait
        let debug_str = format!("{:?}", instance);
        assert!(debug_str.contains("TestStructWithGenericInMacro"));

        // Test Clone trait
        let cloned = instance.clone();
        assert_eq!(cloned.vec_field.len(), 3);
        assert_eq!(cloned.option_field.unwrap(), 42);
    }

    #[test]
    fn test_enum_with_generic_in_macro() {
        let variants = vec![
            TestEnumWithGenericInMacro::Vec(vec!["a".to_string(), "b".to_string()]),
            TestEnumWithGenericInMacro::Option(Some("test".to_string())),
        ];

        // Test Debug trait
        let debug_str = format!("{:?}", variants[0]);
        assert!(debug_str.contains("Vec"));

        // Test Clone trait
        let cloned = variants.clone();
        assert_eq!(cloned.len(), 2);

        // Test pattern matching
        match &variants[0] {
            TestEnumWithGenericInMacro::Vec(vec) => {
                assert_eq!(vec.len(), 2);
                assert_eq!(vec[0], "a");
            }
            _ => panic!("Expected Vec variant"),
        }
    }

    #[test]
    fn test_mixed_struct() {
        let instance = TestStructMixed {
            concrete_field: vec!["hello".to_string(), "world".to_string()],
            generic_field: 42i32,
        };

        // Test Debug trait
        let debug_str = format!("{:?}", instance);
        assert!(debug_str.contains("TestStructMixed"));

        // Test Clone trait
        let cloned = instance.clone();
        assert_eq!(cloned.concrete_field.len(), 2);
        assert_eq!(cloned.generic_field, 42);
    }
}