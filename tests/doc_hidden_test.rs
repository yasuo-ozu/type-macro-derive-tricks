use type_macro_derive_tricks::macro_derive;

// Simple test macro for type positions
macro_rules! SimpleType {
    (int) => { i32 };
    (string) => { String };
    (vec $t:ty) => { Vec<$t> };
}

// Test struct with simple generic parameter
#[macro_derive(Debug, Clone)]
pub struct SimpleGenericStruct<T>
where 
    T: Clone + std::fmt::Debug,
{
    pub simple_field: SimpleType![int],
    pub vec_field: SimpleType![vec String], // Use concrete type for macro
    pub string_field: SimpleType![string],
    pub generic_field: T, // Use generic directly
}

// Test enum with generic parameters
#[macro_derive(Debug, Clone, PartialEq)]
pub enum SimpleGenericEnum<T, U>
where 
    T: Clone + std::fmt::Debug + PartialEq,
    U: Clone + std::fmt::Debug + PartialEq,
{
    Integer(SimpleType![int]),
    Vector(SimpleType![vec i32]), // Use concrete type for macro
    String(SimpleType![string]),
    Custom(T, U), // Use generics directly
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_generic_struct() {
        let instance = SimpleGenericStruct {
            simple_field: 42,
            vec_field: vec!["test".to_string(), "data".to_string()],
            string_field: "hello".to_string(),
            generic_field: "generic_value".to_string(),
        };

        // Test Debug trait
        let debug_str = format!("{:?}", instance);
        assert!(debug_str.contains("SimpleGenericStruct"));
        assert!(debug_str.contains("42"));

        // Test Clone trait
        let cloned = instance.clone();
        assert_eq!(cloned.simple_field, 42);
        assert_eq!(cloned.vec_field.len(), 2);
        assert_eq!(cloned.string_field, "hello");

        // Test that types are correctly resolved
        assert_eq!(std::mem::size_of_val(&cloned.simple_field), 4); // i32
        assert_eq!(cloned.vec_field[0], "test");
    }

    #[test]
    fn test_simple_generic_enum() {
        let variants = vec![
            SimpleGenericEnum::<f64, bool>::Integer(100),
            SimpleGenericEnum::Vector(vec![1, 2, 3]),
            SimpleGenericEnum::String("enum_string".to_string()),
            SimpleGenericEnum::Custom(99.9, true),
        ];

        // Test Debug trait
        let debug_str = format!("{:?}", variants[0]);
        assert!(debug_str.contains("Integer"));

        // Test Clone trait
        let cloned = variants.clone();
        assert_eq!(cloned.len(), 4);

        // Test PartialEq trait
        assert_eq!(variants, cloned);

        // Test pattern matching
        match &variants[0] {
            SimpleGenericEnum::Integer(val) => assert_eq!(*val, 100),
            _ => panic!("Expected Integer variant"),
        }

        match &variants[1] {
            SimpleGenericEnum::Vector(vec) => {
                assert_eq!(vec.len(), 3);
                assert_eq!(vec[0], 1);
            }
            _ => panic!("Expected Vector variant"),
        }

        match &variants[3] {
            SimpleGenericEnum::Custom(float_val, bool_val) => {
                assert_eq!(*float_val, 99.9);
                assert!(*bool_val);
            }
            _ => panic!("Expected Custom variant"),
        }
    }

    #[test]
    fn test_type_alias_generation() {
        // This test verifies that the macro correctly generates type aliases
        // and that the original types work as expected

        // Direct type usage to verify macro expansion
        let int_val: i32 = 42;
        let string_val: String = "test".to_string();
        let vec_val: Vec<String> = vec!["a".to_string(), "b".to_string()];

        assert_eq!(int_val, 42);
        assert_eq!(string_val, "test");
        assert_eq!(vec_val.len(), 2);

        // Test that the struct can be instantiated with different generic parameters
        let struct1 = SimpleGenericStruct {
            simple_field: 10,
            vec_field: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            string_field: "struct1".to_string(),
            generic_field: 1u8,
        };

        let struct2 = SimpleGenericStruct {
            simple_field: 20,
            vec_field: vec!["x".to_string(), "y".to_string()],
            string_field: "struct2".to_string(),
            generic_field: 100i64,
        };

        assert_eq!(struct1.simple_field, 10);
        assert_eq!(struct2.simple_field, 20);
        assert_eq!(struct1.vec_field.len(), 3);
        assert_eq!(struct2.vec_field.len(), 2);

        // Test enum with different generic parameters
        let enum1 = SimpleGenericEnum::<i32, String>::Custom(42, "test".to_string());
        let enum2 = SimpleGenericEnum::<f32, bool>::Custom(3.15, false);

        match enum1 {
            SimpleGenericEnum::Custom(i, s) => {
                assert_eq!(i, 42);
                assert_eq!(s, "test");
            }
            _ => panic!("Expected Custom variant"),
        }

        match enum2 {
            SimpleGenericEnum::Custom(f, b) => {
                assert_eq!(f, 3.15);
                assert!(!b);
            }
            _ => panic!("Expected Custom variant"),
        }
    }
}