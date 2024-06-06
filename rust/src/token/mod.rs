
#[derive(Debug, PartialEq)]
pub enum Token {
    EOF, // end of file
    Keyword(String), // let, return, fn
    Identifier(String), // x, y, foobar

    LeftParen,
    RightParen,
    Semicolon, // ;
    EqualSign, // =

    Int(i32), // an int: 5, 10, 42, etc. Treating all ints as i32 for simplicity
}
