// Debug test to understand token parsing issues
use std::collections::HashMap;
use type_macro_derive_tricks::macro_derive;

macro_rules! TypeMap {
    ($k:ty, $v:ty) => { HashMap<$k, $v> };
}

macro_rules! TypeResult {
    ($t:ty, $e:ty) => { Result<$t, $e> };
}

// This should work fine - simple case
#[macro_derive(Debug)]
pub struct SimpleCase<'a, T, U> {
    pub simple_map: TypeMap![&'a T, U],
}

// This shows the problem - nested macro with lifetime reference
#[macro_derive(Debug)]
pub struct ProblematicCase<'a, T, U> {
    pub complex: TypeMap![&'a str, TypeResult![T, U]],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_case() {
        let simple = SimpleCase::<i32, String> {
            simple_map: HashMap::new(),
        };
        println!("{:?}", simple);
    }

    #[test]
    fn test_problematic_case() {
        let prob = ProblematicCase::<i32, String> {
            complex: HashMap::new(),
        };
        println!("{:?}", prob);
    }
}
