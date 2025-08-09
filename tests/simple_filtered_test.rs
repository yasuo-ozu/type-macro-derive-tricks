use type_macro_derive_tricks::macro_derive;

// Simple macros for testing
macro_rules! SimpleVec {
    ($t:ty) => { Vec<$t> };
}

macro_rules! SimpleOption {
    ($t:ty) => { Option<$t> };
}

macro_rules! NoGeneric {
    () => { String };
}

// Test struct where some macros use concrete types (don't need generics)
// This should work because V is used directly, so it's not unused
#[macro_derive(Debug, Clone)]
pub struct TestStruct<T, U, V>
where
    T: Clone + std::fmt::Debug,
    U: Clone + std::fmt::Debug,  
    V: Clone + std::fmt::Debug,
{
    pub concrete_vec: SimpleVec![i32],        // Macro with concrete type - no generics needed
    pub concrete_option: SimpleOption![String], // Macro with concrete type - no generics needed
    pub no_generic: NoGeneric![],             // Macro with no generics - no generics needed
    pub direct_t: T,                          // Direct usage of T
    pub direct_u: U,                          // Direct usage of U
    pub direct_v: V,                          // Direct usage of V to avoid unused error
}

// Test enum with similar pattern
#[macro_derive(Debug, Clone, PartialEq)]
pub enum TestEnum<T, U, V>
where
    T: Clone + std::fmt::Debug + PartialEq,
    U: Clone + std::fmt::Debug + PartialEq,
    V: Clone + std::fmt::Debug + PartialEq,
{
    ConcreteVec(SimpleVec![f64]),        // Macro with concrete type
    ConcreteOption(SimpleOption![bool]), // Macro with concrete type
    NoGeneric(NoGeneric![]),             // Macro with no generics
    DirectT(T),                          // Direct T usage
    DirectU(U),                          // Direct U usage
    DirectV(V),                          // Direct V usage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_compiles() {
        let instance = TestStruct {
            concrete_vec: vec![1, 2, 3],
            concrete_option: Some("test".to_string()),
            no_generic: "constant".to_string(),
            direct_t: 42i32,
            direct_u: true,
            direct_v: 3.15f64,
        };

        // Test Debug trait works
        let debug_str = format!("{:?}", instance);
        assert!(debug_str.contains("TestStruct"));

        // Test Clone trait works
        let cloned = instance.clone();
        assert_eq!(cloned.concrete_vec.len(), 3);
        assert_eq!(cloned.concrete_option.as_ref().unwrap(), "test");
        assert_eq!(cloned.no_generic, "constant");
        assert_eq!(cloned.direct_t, 42);
        assert!(cloned.direct_u);
        assert_eq!(cloned.direct_v, 3.15);
    }

    #[test]
    fn test_enum_compiles() {
        let variants = vec![
            TestEnum::<i32, String, f64>::ConcreteVec(vec![1.1, 2.2, 3.3]),
            TestEnum::ConcreteOption(Some(true)),
            TestEnum::NoGeneric("constant".to_string()),
            TestEnum::DirectT(100),
            TestEnum::DirectU("direct".to_string()),
            TestEnum::DirectV(3.15),
        ];

        // Test Debug trait works
        let debug_str = format!("{:?}", variants[0]);
        assert!(debug_str.contains("ConcreteVec"));

        // Test Clone trait works
        let cloned = variants.clone();
        assert_eq!(cloned.len(), 6);

        // Test PartialEq trait works
        assert_eq!(variants, cloned);

        // Test pattern matching works
        match &variants[0] {
            TestEnum::ConcreteVec(vec) => {
                assert_eq!(vec.len(), 3);
                assert_eq!(vec[0], 1.1);
            }
            _ => panic!("Expected ConcreteVec variant"),
        }
    }

    #[test]
    fn test_different_generic_instantiations() {
        // Test that we can instantiate with different generic parameters
        let struct1 = TestStruct::<i32, String, bool> {
            concrete_vec: vec![1, 2],
            concrete_option: Some("a".to_string()),
            no_generic: "test".to_string(),
            direct_t: 10,
            direct_u: "hello".to_string(),
            direct_v: true,
        };

        let struct2 = TestStruct::<f64, bool, String> {
            concrete_vec: vec![3, 4], // Same concrete type despite different T
            concrete_option: Some("b".to_string()), // Same concrete type despite different U  
            no_generic: "test".to_string(),
            direct_t: 3.15,
            direct_u: false,
            direct_v: "different".to_string(),
        };

        // Verify type consistency - the macro types are the same regardless of generics
        assert_eq!(struct1.concrete_vec, vec![1, 2]);
        assert_eq!(struct2.concrete_vec, vec![3, 4]);
        
        // But direct generic usage is different
        assert_eq!(struct1.direct_t, 10i32);
        assert_eq!(struct2.direct_t, 3.15f64);
        assert_eq!(struct1.direct_u, "hello".to_string());
        assert!(!struct2.direct_u);
        assert!(struct1.direct_v);
        assert_eq!(struct2.direct_v, "different".to_string());
    }
}