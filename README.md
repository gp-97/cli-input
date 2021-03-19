[![License: MIT](https://img.shields.io/badge/License-MIT-g.svg)](https://opensource.org/licenses/MIT)
# cli-input

Rust doesn't provide simple syntax for getting user-input from the terminal.

__cli-input__ is an attempt in the same direction to provide pythonic syntax for user-inputs.

### Currently supports:
- *unsigned* : `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- *integer* : `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- *float* : `f32`, `f64`
- *String*: `String` ([std::string::String](https://doc.rust-lang.org/std/string/struct.String.html))

### Examples:
> Note: first import the `user_inp.rs` module in your file
```sh
    let var: usize = input::<usize>();
    let var: i32 = input::<i32>();
    let var: f32 = input::<f32>();
    let var: String = input::<String>();
```
