// Test to demonstrate the lifetime parameter handling issue with type macros
use type_macro_derive_tricks::macro_derive;

// Macro that uses lifetime parameters
macro_rules! RefMacro {
    ($t:ty, $lt:lifetime) => { &$lt $t };
}

// Macro that doesn't use lifetimes
macro_rules! SimpleMacro {
    ($t:ty) => { Vec<$t> };
}

// This should work - no lifetime in macro
#[macro_derive(Debug, Clone)]
pub struct WorkingStruct<'a, T> {
    pub simple: SimpleMacro![T],
    pub reference: &'a T,
}

// This demonstrates the problem - macro uses lifetime
#[macro_derive(Debug, Clone)]
pub struct ProblematicStruct<'a, T> {
    pub reference_macro: RefMacro![T, 'a],
    pub simple: SimpleMacro![T],
}

// Test with multiple lifetimes
macro_rules! ComplexRefMacro {
    ($t:ty, $lt1:lifetime, $lt2:lifetime) => { (&$lt1 $t, &$lt2 $t) };
}

#[macro_derive(Debug, Clone)]
pub struct MultipleLifetimes<'a, 'b, T> {
    pub complex_ref: ComplexRefMacro![T, 'a, 'b],
    pub simple_ref: &'a T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_struct() {
        let data = 42i32;
        let working = WorkingStruct {
            simple: vec![1, 2, 3],
            reference: &data,
        };
        
        println!("{:?}", working);
        let _cloned = working.clone();
    }

    #[test]
    fn test_problematic_struct() {
        let data = 42i32;
        let problematic = ProblematicStruct {
            reference_macro: &data,
            simple: vec![1, 2, 3],
        };
        
        println!("{:?}", problematic);
        let _cloned = problematic.clone();
    }

    #[test]
    fn test_multiple_lifetimes() {
        let data1 = 42i32;
        let data2 = 24i32;
        let multiple = MultipleLifetimes {
            complex_ref: (&data1, &data2),
            simple_ref: &data1,
        };
        
        println!("{:?}", multiple);
        let _cloned = multiple.clone();
    }
}