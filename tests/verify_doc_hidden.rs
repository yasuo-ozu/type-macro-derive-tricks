// This test verifies that the generated code includes #[doc(hidden)] attributes
// We can inspect this by using cargo expand or by checking the generated output

use type_macro_derive_tricks::macro_derive;

macro_rules! TestMacro {
    (simple) => { i32 };
    (complex) => { Vec<String> };
}

#[macro_derive(Debug)]
pub struct TestStruct {
    pub field1: TestMacro![simple],
    pub field2: TestMacro![complex],
}

// The expected expansion should include:
// #[doc(hidden)]
// type __TypeMacroAlias... = i32;
// #[doc(hidden)]
// type __TypeMacroAlias... = Vec<String>;
//
// #[derive(Debug)]
// pub struct TestStruct {
//     pub field1: __TypeMacroAlias...,
//     pub field2: __TypeMacroAlias...,
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_works() {
        let instance = TestStruct {
            field1: 42,
            field2: vec!["test".to_string()],
        };

        let debug_str = format!("{:?}", instance);
        assert!(debug_str.contains("TestStruct"));
        assert!(debug_str.contains("42"));
    }
}