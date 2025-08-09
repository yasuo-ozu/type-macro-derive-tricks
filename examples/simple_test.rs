#![allow(private_bounds, private_interfaces)]
use type_macro_derive_tricks::macro_derive;

// Mock the Symbol macro for testing
macro_rules! Symbol {
    ($lit:literal) => {
        ()
    };
}

// Mock the WithSpan type for testing
#[derive(Debug, Clone, PartialEq)]
struct WithSpan<T, S> {
    value: T,
    span: S,
}

// Mock Span trait
trait Span: Default + Clone {}
impl Span for () {}

// Test a simple case
#[macro_derive(Debug, Clone)]
pub struct SimpleTest<S: Span> {
    pub token: WithSpan<Symbol!["+"], S>,
}

fn main() {
    println!("Macro expansion test completed!");
}