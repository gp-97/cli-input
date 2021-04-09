[![License: MIT](https://img.shields.io/badge/License-MIT-g.svg)](https://opensource.org/licenses/MIT)
# term

Rust doesn't provide simple syntax for getting user-input from the terminal.

__term__ is an attempt in the same direction to provide pythonic syntax for user-inputs.

### Currently supports:
- *unsigned* : `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- *integer* : `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- *float* : `f32`, `f64`
- *String*: `String` ([std::string::String](https://doc.rust-lang.org/std/string/struct.String.html)), `&str` (Via Workaround)
> Note: *&str* works via a workaround as mentioned in __Examples__ section.

### Examples:
```sh
    use term::input;

    let var: usize = input::<usize>();
    let var: i32 = input::<i32>();
    let var: f32 = input::<f32>();
    let var: String = input::<String>();
```
In case `&str` is required:
```sh
    use term::input;

    let var = input::<String>();
    let var = var.as_str();
```
