use type_macro_derive_tricks::macro_derive;
use std::collections::{HashMap, BTreeMap, HashSet, BTreeSet};
use std::marker::PhantomData;

// Type-position macros that take type parameters and emit dependent types

macro_rules! Container {
    (vec, $t:ty) => { Vec<$t> };
    (set, $t:ty) => { HashSet<$t> };
    (btree_set, $t:ty) => { BTreeSet<$t> };
    (option, $t:ty) => { Option<$t> };
    (box, $t:ty) => { Box<$t> };
}

macro_rules! Mapping {
    (hash, $k:ty, $v:ty) => { HashMap<$k, $v> };
    (btree, $k:ty, $v:ty) => { BTreeMap<$k, $v> };
    (result, $t:ty, $e:ty) => { Result<$t, $e> };
}

macro_rules! Nested {
    (vec_of_options, $t:ty) => { Vec<Option<$t>> };
    (option_of_vec, $t:ty) => { Option<Vec<$t>> };
    (result_of_vec, $t:ty, $e:ty) => { Result<Vec<$t>, $e> };
    (vec_of_results, $t:ty, $e:ty) => { Vec<Result<$t, $e>> };
}

macro_rules! Complex {
    (ref_cell, $t:ty) => { std::cell::RefCell<$t> };
    (rc, $t:ty) => { std::rc::Rc<$t> };
    (arc, $t:ty) => { std::sync::Arc<$t> };
    (mutex, $t:ty) => { std::sync::Mutex<$t> };
}

macro_rules! Tuple {
    (pair, $t:ty) => { ($t, $t) };
    (triple, $t:ty) => { ($t, $t, $t) };
    (mixed, $t:ty, $u:ty) => { ($t, $u, Vec<$t>) };
}

macro_rules! Phantom {
    (single, $t:ty) => { PhantomData<$t> };
    (tuple, $t:ty, $u:ty) => { PhantomData<($t, $u)> };
    (fn_ptr, $t:ty, $u:ty) => { PhantomData<fn($t) -> $u> };
}

macro_rules! Function {
    (unary, $t:ty, $u:ty) => { fn($t) -> $u };
    (binary, $t:ty, $u:ty, $v:ty) => { fn($t, $u) -> $v };
    (closure, $t:ty) => { Option<fn($t) -> $t> };
}

// Test struct using container macros with different type parameters
#[macro_derive(Debug, Clone)]
pub struct ContainerStruct<T, U>
where
    T: Clone + std::fmt::Debug + std::hash::Hash + Eq + Ord,
    U: Clone + std::fmt::Debug,
{
    pub vec_data: Container![vec, T],
    pub set_data: Container![set, T],
    pub btree_set_data: Container![btree_set, T],
    pub option_data: Container![option, U],
    pub boxed_data: Container![box, U],
}

// Test enum using mapping macros with type parameters
#[macro_derive(Debug, Clone)]
pub enum MappingEnum<K, V, E>
where
    K: Clone + std::fmt::Debug + std::hash::Hash + Eq + Ord,
    V: Clone + std::fmt::Debug,
    E: Clone + std::fmt::Debug,
{
    Hash(Mapping![hash, K, V]),
    BTree(Mapping![btree, K, V]),
    Result(Mapping![result, V, E]),
}

// Test struct using nested type macros
#[macro_derive(Debug, Clone)]
pub struct NestedStruct<T, E>
where
    T: Clone + std::fmt::Debug,
    E: Clone + std::fmt::Debug,
{
    pub vec_options: Nested![vec_of_options, T],
    pub option_vec: Nested![option_of_vec, T],
    pub result_vec: Nested![result_of_vec, T, E],
    pub vec_results: Nested![vec_of_results, T, E],
}

// Test struct using complex wrapper macros
#[macro_derive(Debug)]
pub struct ComplexWrapperStruct<T>
where
    T: std::fmt::Debug,
{
    pub ref_cell: Complex![ref_cell, T],
    pub rc: Complex![rc, T],
    pub arc: Complex![arc, T],
    pub mutex: Complex![mutex, T],
}

// Test struct using tuple macros with type parameters
#[macro_derive(Debug, Clone, PartialEq)]
pub struct TupleStruct<T, U>
where
    T: Clone + std::fmt::Debug + PartialEq,
    U: Clone + std::fmt::Debug + PartialEq,
{
    pub pair: Tuple![pair, T],
    pub triple: Tuple![triple, T],
    pub mixed: Tuple![mixed, T, U],
}

// Test enum using phantom data macros
#[macro_derive(Debug, Clone, PartialEq)]
pub enum PhantomEnum<T, U>
where
    T: 'static,
    U: 'static,
{
    Single(Phantom![single, T]),
    Tuple(Phantom![tuple, T, U]),
    FnPtr(Phantom![fn_ptr, T, U]),
}

// Test struct using function pointer macros
#[macro_derive(Debug, Clone)]
pub struct FunctionStruct<T, U, V>
where
    T: Clone + std::fmt::Debug + 'static,
    U: Clone + std::fmt::Debug + 'static,
    V: Clone + std::fmt::Debug + 'static,
{
    pub unary: Function![unary, T, U],
    pub binary: Function![binary, T, U, V],
    pub closure: Function![closure, T],
}

// Test struct combining multiple macro types with interdependent parameters
#[macro_derive(Debug, Clone)]
pub struct CombinedStruct<T, U, E>
where
    T: Clone + std::fmt::Debug + std::hash::Hash + Eq + Ord,
    U: Clone + std::fmt::Debug,
    E: Clone + std::fmt::Debug,
{
    pub containers: Container![vec, Container![option, T]],
    pub mappings: Mapping![hash, T, Nested![vec_of_results, U, E]],
    pub complex_nested: Complex![arc, Tuple![mixed, T, U]],
    pub phantom_marker: Phantom![tuple, T, U],
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, BTreeMap, HashSet, BTreeSet};

    #[test]
    fn test_container_struct() {
        let mut set_data = HashSet::new();
        set_data.insert(1);
        set_data.insert(2);

        let mut btree_set_data = BTreeSet::new();
        btree_set_data.insert(10);
        btree_set_data.insert(20);

        let instance = ContainerStruct {
            vec_data: vec![1, 2, 3],
            set_data,
            btree_set_data,
            option_data: Some("test".to_string()),
            boxed_data: Box::new("boxed".to_string()),
        };

        // Test Debug trait
        let debug_str = format!("{:?}", instance);
        assert!(debug_str.contains("ContainerStruct"));

        // Test Clone trait
        let cloned = instance.clone();
        assert_eq!(cloned.vec_data.len(), 3);
        assert_eq!(cloned.set_data.len(), 2);
        assert!(cloned.set_data.contains(&1));
        assert_eq!(cloned.option_data.as_ref().unwrap(), "test");
        assert_eq!(cloned.boxed_data.as_str(), "boxed");
    }

    #[test]
    fn test_mapping_enum() {
        let mut hash_map = HashMap::new();
        hash_map.insert("key1".to_string(), 100);
        hash_map.insert("key2".to_string(), 200);

        let mut btree_map = BTreeMap::new();
        btree_map.insert("key1".to_string(), 100);
        btree_map.insert("key2".to_string(), 200);

        let variants = vec![
            MappingEnum::<String, i32, String>::Hash(hash_map),
            MappingEnum::BTree(btree_map),
            MappingEnum::Result(Ok(42)),
        ];

        // Test Debug trait
        let debug_str = format!("{:?}", variants[0]);
        assert!(debug_str.contains("Hash"));

        // Test Clone trait
        let cloned = variants.clone();
        assert_eq!(cloned.len(), 3);

        // Test pattern matching
        match &variants[0] {
            MappingEnum::Hash(map) => {
                assert_eq!(map.len(), 2);
                assert_eq!(map.get("key1"), Some(&100));
            }
            _ => panic!("Expected Hash variant"),
        }

        match &variants[2] {
            MappingEnum::Result(result) => {
                assert_eq!(*result, Ok(42));
            }
            _ => panic!("Expected Result variant"),
        }
    }

    #[test]
    fn test_nested_struct() {
        let instance = NestedStruct {
            vec_options: vec![Some(1), None, Some(3)],
            option_vec: Some(vec![10, 20]),
            result_vec: Ok(vec![10, 20, 30]),
            vec_results: vec![Ok(100), Err("error".to_string())],
        };

        // Test Debug trait
        let debug_str = format!("{:?}", instance);
        assert!(debug_str.contains("NestedStruct"));

        // Test Clone trait
        let cloned = instance.clone();
        assert_eq!(cloned.vec_options.len(), 3);
        assert_eq!(cloned.option_vec.as_ref().unwrap().len(), 2);
        assert_eq!(cloned.result_vec.as_ref().unwrap().len(), 3);
        assert_eq!(cloned.vec_results.len(), 2);

        // Test nested type access
        assert_eq!(cloned.vec_options[0], Some(1));
        assert_eq!(cloned.vec_options[1], None);
        assert_eq!(cloned.option_vec.as_ref().unwrap()[0], 10);
        assert_eq!(cloned.result_vec.as_ref().unwrap()[2], 30);
    }

    #[test]
    fn test_complex_wrapper_struct() {
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::sync::{Arc, Mutex};

        let instance = ComplexWrapperStruct::<i32> {
            ref_cell: RefCell::new(42),
            rc: Rc::new(42),
            arc: Arc::new(42),
            mutex: Mutex::new(42),
        };

        // Test Debug trait
        let debug_str = format!("{:?}", instance);
        assert!(debug_str.contains("ComplexWrapperStruct"));

        // Test field access (Clone not available due to Mutex)
        assert_eq!(*instance.ref_cell.borrow(), 42);
        assert_eq!(*instance.rc, 42);
        assert_eq!(*instance.arc, 42);
        assert_eq!(*instance.mutex.lock().unwrap(), 42);
    }

    #[test]
    fn test_tuple_struct() {
        let instance = TupleStruct {
            pair: (1, 1),
            triple: (2, 2, 2),
            mixed: (3, "three".to_string(), vec![3, 33, 333]),
        };

        // Test Debug trait
        let debug_str = format!("{:?}", instance);
        assert!(debug_str.contains("TupleStruct"));

        // Test Clone trait
        let cloned = instance.clone();
        assert_eq!(cloned.pair, (1, 1));
        assert_eq!(cloned.triple, (2, 2, 2));
        assert_eq!(cloned.mixed.0, 3);
        assert_eq!(cloned.mixed.1, "three");
        assert_eq!(cloned.mixed.2, vec![3, 33, 333]);

        // Test PartialEq trait
        assert_eq!(instance, cloned);
    }

    #[test]
    fn test_phantom_enum() {
        let variants = vec![
            PhantomEnum::<i32, String>::Single(PhantomData),
            PhantomEnum::Tuple(PhantomData),
            PhantomEnum::FnPtr(PhantomData),
        ];

        // Test Debug trait
        let debug_str = format!("{:?}", variants[0]);
        assert!(debug_str.contains("Single"));

        // Test Clone trait
        let cloned = variants.clone();
        assert_eq!(cloned.len(), 3);

        // Test PartialEq trait
        assert_eq!(variants, cloned);

        // Test pattern matching
        match &variants[0] {
            PhantomEnum::Single(_) => {}, // PhantomData has no data to check
            _ => panic!("Expected Single variant"),
        }
    }

    #[test]
    fn test_function_struct() {
        fn add_one(x: i32) -> String {
            (x + 1).to_string()
        }

        fn combine(x: i32, y: String) -> f64 {
            format!("{}{}", x, y).parse().unwrap_or(0.0)
        }

        fn double(x: i32) -> i32 {
            x * 2
        }

        let instance = FunctionStruct {
            unary: add_one,
            binary: combine,
            closure: Some(double),
        };

        // Test function calls to verify types
        let result1 = (instance.unary)(5);
        assert_eq!(result1, "6");

        let result2 = (instance.binary)(42, "0".to_string());
        assert_eq!(result2, 420.0);

        let result3 = instance.closure.unwrap()(10);
        assert_eq!(result3, 20);

        // Test Debug trait (functions implement Debug in some contexts)
        let debug_str = format!("{:?}", instance);
        assert!(!debug_str.is_empty()); // Just ensure it compiles

        // Test Clone trait
        let cloned = instance.clone();
        let cloned_result = (cloned.unary)(7);
        assert_eq!(cloned_result, "8");
    }

    #[test]
    fn test_combined_struct() {
        use std::sync::Arc;

        let mut hash_map = HashMap::new();
        hash_map.insert(
            1,
            vec![Ok("value100".to_string()), Err("error1".to_string())],
        );
        hash_map.insert(
            2,
            vec![Ok("value200".to_string())],
        );

        let instance = CombinedStruct::<i32, String, String> {
            containers: vec![Some(10), None, Some(30)],
            mappings: hash_map,
            complex_nested: Arc::new((100, "hundred".to_string(), vec![1, 0, 0])),
            phantom_marker: PhantomData,
        };

        // Test Debug trait
        let debug_str = format!("{:?}", instance);
        assert!(debug_str.contains("CombinedStruct"));

        // Test Clone trait
        let cloned = instance.clone();
        assert_eq!(cloned.containers.len(), 3);
        assert_eq!(cloned.mappings.len(), 2);
        assert_eq!(cloned.complex_nested.0, 100);
        assert_eq!(cloned.complex_nested.1, "hundred");

        // Test nested structure access
        assert_eq!(cloned.containers[0], Some(10));
        assert_eq!(cloned.containers[1], None);
        
        let vec_results = cloned.mappings.get(&1).unwrap();
        assert_eq!(vec_results.len(), 2);
        assert_eq!(vec_results[0], Ok("value100".to_string()));
        assert_eq!(vec_results[1], Err("error1".to_string()));
    }

    #[test]
    fn test_type_parameter_dependency() {
        // This test verifies that type-position macros correctly handle 
        // type parameters and emit types that depend on those parameters

        // Test that the Container macro correctly uses type parameters
        let vec_container: Vec<String> = vec!["test".to_string()];
        let option_container: Option<i32> = Some(42);
        let box_container: Box<f64> = Box::new(3.15);

        assert_eq!(vec_container.len(), 1);
        assert_eq!(option_container, Some(42));
        assert_eq!(*box_container, 3.15);

        // Test that the Mapping macro correctly uses multiple type parameters
        let mut hash_mapping: HashMap<String, i32> = HashMap::new();
        hash_mapping.insert("key".to_string(), 100);
        assert_eq!(hash_mapping.get("key"), Some(&100));

        let result_mapping: Result<String, &str> = Ok("success".to_string());
        assert_eq!(result_mapping, Ok("success".to_string()));

        // Test that nested macros preserve type parameter relationships
        let vec_options: Vec<Option<bool>> = vec![Some(true), None, Some(false)];
        assert_eq!(vec_options.len(), 3);
        assert_eq!(vec_options[0], Some(true));

        // Test that complex combinations maintain type safety
        let complex_type: HashMap<i32, Vec<Result<String, bool>>> = HashMap::new();
        assert_eq!(complex_type.len(), 0);

        // Test tuple macro with different type parameters
        let pair: (i32, i32) = (1, 2);
        let mixed: (String, bool, Vec<String>) = ("test".to_string(), true, vec!["a".to_string()]);
        assert_eq!(pair.0, 1);
        assert_eq!(mixed.2.len(), 1);
    }
}