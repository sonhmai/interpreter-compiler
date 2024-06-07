# String

Exploring fascinating `string` topics in the programming world across different languages.

- Unicode Scalar Value (Rust) / Unicode Code Point (Go)
- Index and Loop
  - Rust: cannot index or loop a String
  - Go: index a String gives byte, can loop over byte or UTF8 rune (code point)

## Go
- a string holds arbitrary bytes
- byte in str `not required` to be Unicode/ UTF8 text or any other format.
- indexing string gives bytes, not characters.
- character is an ambiguous concept and `should not` be used to describe string.
- string # string literal
  - `string literal` always contain UTF8 if there is not `byte-level escape \x`
  - Go source code is always `UTF8`
- Unicode # UTF8
  - Unicode is a code point system.
  - UTF-8 is one way to represent Unicode system in bytes. Other ways are `UTF16`, etc.
  - Unicode code point U+2318, with hexadecimal value 2318, represents the symbol ⌘
- Char concept is ambiguous
  - a char can be represented by different sequences of code points 
  -> different UTF8 bytes
- `rune` = `code point`
- `for` vs `for range` on string
  - for range decodes one `UTF8-encoded rune` in each iteration

```go
const noEscape = "bdb2" // UTF8
const withEscape = "\xbd\xb2" // not UTF8
// a string literal with peculiar bytes with byte-level escape, not unicode
const sample = "\xbd\xb2\x3d\xbc\x20\xe2\x8c\x98" 
string[3] -> returns byte, not char

// loop over each byte and print out byte in hex.
// Bytes are not valid char so print them in string format gives rubbish like ��=�
for i:=0; i < len(sample); i++ {
    fmt.Printf("%x ", sample[i])
}
// stdout: bd b2 3d bc 20 e2 8c 98

// package unicode/utf8 -> utils for validate, put together, taken apart UTF-8 strings
const nihongo = "日本語"
for i, w := 0, 0; i < len(nihongo); i += w {
    runeValue, width := utf8.DecodeRuneInString(nihongo[i:])
    fmt.Printf("%#U starts at byte position %d\n", runeValue, i)
    w = width
}
/* Output
U+65E5 '日' starts at byte position 0
U+672C '本' starts at byte position 3
U+8A9E '語' starts at byte position 6
*/
```


## References
- [Rob Pike - Strings, bytes, runes and characters in Go](https://go.dev/blog/strings)
- [The Absolute Minimum Every Software Developer Absolutely, Positively Must Know About Unicode and Character Sets](http://www.joelonsoftware.com/articles/Unicode.html)
- [What’s the difference between a Rust char and a Go rune?](https://christianfscott.com/rust-chars-vs-go-runes/)
- https://stackoverflow.com/questions/48465265/what-is-the-difference-between-unicode-code-points-and-unicode-scalars