// Do not edit: This code was generated by flatdata's generator.
pub mod a {

pub mod schema {
pub mod structs {}}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bar {
    Value = 0,
}

impl flatdata::helper::Int for Bar {
    const IS_SIGNED: bool = false;
}
}

pub mod b {

pub mod schema {
pub mod structs {}}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bar {
    Value = 0,
}

impl flatdata::helper::Int for Bar {
    const IS_SIGNED: bool = false;
}
}

pub mod n {

pub mod schema {
pub mod structs {
pub const FOO: &str = r#"namespace a {
enum Bar : u8
{
    VALUE = 0,
}
}

namespace n {
struct Foo
{
    f : .a.Bar : 8;
}
}

"#;}}

define_struct!(
    Foo,
    RefFoo,
    RefMutFoo,
    schema::structs::FOO,
    1,
    (f, set_f, super::a::Bar, u8, 0, 8));
}

pub mod m {

pub mod schema {
pub mod structs {
pub const FOO: &str = r#"namespace b {
enum Bar : u8
{
    VALUE = 0,
}
}

namespace m {
struct Foo
{
    f : .b.Bar : 8;
}
}

"#;}}

define_struct!(
    Foo,
    RefFoo,
    RefMutFoo,
    schema::structs::FOO,
    1,
    (f, set_f, super::b::Bar, u8, 0, 8));
}
