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

// Mock Parse, Unparse, Spanned traits
#[allow(dead_code)]
trait Parse<Atom> {
    type Error;
    fn parse(stream: impl IntoParseStream<Atom = Atom>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

#[allow(dead_code)]
trait Unparse<Atom> {
    fn unparse<SS: Emitter<Atom>>(&self, sink: &mut SS) -> Result<(), SS::Error>;
}

#[allow(dead_code)]
trait Spanned {
    type Span;
    fn span(&self) -> Self::Span;
}

// Mock supporting types
#[allow(dead_code)]
trait IntoParseStream {
    type Atom;
}

#[allow(dead_code)]
trait Emitter<Atom> {
    type Error;
}

// Now test the macro
#[macro_derive(Debug, Clone)]
pub enum BinOp<S: Span> {
    Add(WithSpan<Symbol!["+"], S>),
    Sub(WithSpan<Symbol!["-"], S>),
    Mul(WithSpan<Symbol!["*"], S>),
}

#[macro_derive(Debug, Clone)]
pub struct ItemFn<S: Span> {
    pub fn_token: WithSpan<Symbol!["fn"], S>,
    pub name: String,
    pub span: S,
}

#[test]
fn test_binop_compiles() {
    // If this compiles, our macro worked
    let op = BinOp::<()>::Add(WithSpan { value: (), span: () });
    println!("{:?}", op);
}

#[test]
fn test_item_fn_compiles() {
    // If this compiles, our macro worked
    let item = ItemFn {
        fn_token: WithSpan { value: (), span: () },
        name: "test".to_string(),
        span: (),
    };
    println!("{:?}", item);
}