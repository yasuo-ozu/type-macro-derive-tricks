# type-macro-derive-tricks crate [![Latest Version]][crates.io] [![Documentation]][docs.rs] [![GitHub Actions]][actions]

[Latest Version]: https://img.shields.io/crates/v/type-macro-derive-tricks.svg
[crates.io]: https://crates.io/crates/type-macro-derive-tricks
[Documentation]: https://img.shields.io/docsrs/type-macro-derive-tricks
[docs.rs]: https://docs.rs/type-macro-derive-tricks/latest/
[GitHub Actions]: https://github.com/yasuo-ozu/type-macro-derive-tricks/actions/workflows/rust.yml/badge.svg
[actions]: https://github.com/yasuo-ozu/type-macro-derive-tricks/actions/workflows/rust.yml


This crate provides derive macros that can work with ADTs containing macros in type positions,
which standard Rust derive macros cannot handle.

## Problem

Standard Rust derive macros fail when applied to types containing macro invocations:

```rust,compile_fail
macro_rules! typ {
    ($t:ty) => {($t, $t)};
}

#[derive(Clone)] // Error: derive cannot be used on items with type macros
pub enum MyEnum<S> {
    Add(typ![u32]),
}
```

## Solution

This crate provides a procedural macro that extracts macro types, generates type aliases,
and then applies standard derive macros:

```rust
use type_macro_derive_tricks::macro_derive;

macro_rules! typ {
    ($t:ty) => {($t, $t)};
}

#[macro_derive(Debug)]
pub enum MyEnum {
    Add(typ![i32]),
    Sub(typ![String]),
}
```

This will generate:
```rust
type __RandomName1 = (i32, i32);
type __RandomName2 = (String, String);

#[derive(Debug)]
pub enum MyEnum {
    Add(__RandomName1),
    Sub(__RandomName2),
}
```

## Features

- Works with any derive macro (`Debug`, `Clone`, `PartialEq`, etc.)
- Supports complex generic types with lifetimes
- Handles nested macro invocations
- Generates clean, hidden type aliases
- Maintains proper generic parameter relationships

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
type-macro-derive-tricks = "0.1.0"
```

Then use `#[macro_derive(...)]` instead of `#[derive(...)]` on types containing macro invocations in type positions:

```rust
use type_macro_derive_tricks::macro_derive;

macro_rules! MyType {
    ($t:ty) => { Vec<$t> };
}

#[macro_derive(Debug, Clone, PartialEq)]
pub struct MyStruct<T> {
    pub field: MyType![T],
}
```

## Advanced Examples

### With Lifetimes

```rust
use type_macro_derive_tricks::macro_derive;

macro_rules! RefMap {
    ($k:ty, $v:ty) => { std::collections::HashMap<$k, $v> };
}

#[macro_derive(Debug, Clone)]
pub struct ComplexStruct<'a, T, U> {
    pub data: RefMap![&'a str, Result<T, U>],
}
```

### Nested Macros

```rust
use type_macro_derive_tricks::macro_derive;

macro_rules! Wrapper {
    ($t:ty) => { Box<$t> };
}

macro_rules! Container {
    ($t:ty) => { Vec<$t> };
}

#[macro_derive(Debug)]
pub enum NestedEnum<T> {
    Nested(Container![Wrapper![T]]),
}
```

## How It Works

1. The macro scans the AST for macro invocations in type positions
2. For each unique macro type, it generates a hidden type alias with a random name
3. It replaces all macro invocations with references to these aliases
4. It applies the requested derive traits to the transformed structure
5. Both the type aliases and the derived implementation are output together

## Limitations

- Only works with macros that expand to valid types
- The original macro invocations are replaced in the final code
- Generated type aliases are hidden but still present in the compiled code

## License

This project is licensed under the MIT License - see the LICENSE file for details.