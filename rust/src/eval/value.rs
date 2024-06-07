
// Value is equivalent to Object in monkey language.
// Value as in `value representation` which is the core in evaluation.
// enum was chosen instead of trait to limit values set to a limited number here,
// not open for extension outside of this crate.
#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i32),
    Boolean(bool),
    Null,
}