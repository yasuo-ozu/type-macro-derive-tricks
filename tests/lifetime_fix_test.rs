use type_macro_derive_tricks::macro_derive;
use std::collections::HashMap;

// Type-position macros for testing
macro_rules! TypeMap {
    ($k:ty, $v:ty) => { HashMap<$k, $v> };
}

macro_rules! TypeResult {
    ($t:ty, $e:ty) => { Result<$t, $e> };
}

// Test that the nested macro call with lifetimes works
#[macro_derive(Debug, Clone)]
pub enum TestEnum<'a, T, U>
where
    T: Clone + 'a,
    U: std::fmt::Debug,
{
    Simple(T),
    Complex(TypeMap![&'a str, TypeResult![T, U]]),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetime_in_nested_macros() {
        let test_data = 42i32;
        let _test_ref = &test_data;
        
        // This should compile successfully now
        let mut map = HashMap::new();
        map.insert("key", Ok(100i32));
        
        let instance = TestEnum::<i32, String>::Complex(map);
        
        // Test Debug trait
        let debug_str = format!("{:?}", instance);
        assert!(debug_str.contains("Complex"));
        
        // Test Clone trait
        let cloned = instance.clone();
        
        match cloned {
            TestEnum::Complex(ref map) => {
                assert!(map.contains_key("key"));
            }
            _ => panic!("Expected Complex variant"),
        }
    }
}