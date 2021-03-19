use std::fmt::{Debug, Display};
use std::io;
use std::str::FromStr;

pub fn input<T>() -> T
where
    T: Clone + Display + FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Invalid Input");
    match String::from(inp.trim()).parse::<T>() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("[ERROR]: {:?}", e);
            std::process::exit(1);
        }
    }
}

pub fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn u8_test() {
        let a: u8 = input::<u8>();
        assert_eq!(type_of(&a), "u8");
    }
    #[test]
    fn u16_test() {
        let a: u16 = input::<u16>();
        assert_eq!(type_of(&a), "u16");
    }
    #[test]
    fn u32_test() {
        let a: u32 = input::<u32>();
        assert_eq!(type_of(&a), "u32");
    }
    #[test]
    fn u128_test() {
        let a: u128 = input::<u128>();
        assert_eq!(type_of(&a), "u128");
    }
    #[test]
    fn usize_test() {
        let a: usize = input::<usize>();
        assert_eq!(type_of(&a), "usize");
    }
    #[test]
    fn i8_test() {
        let a: i8 = input::<i8>();
        assert_eq!(type_of(&a), "i8");
    }
    #[test]
    fn i16_test() {
        let a: i16 = input::<i16>();
        assert_eq!(type_of(&a), "i16");
    }
    #[test]
    fn i32_test() {
        let a: i32 = input::<i32>();
        assert_eq!(type_of(&a), "i32");
    }
    #[test]
    fn i64_test() {
        let a: i64 = input::<i64>();
        assert_eq!(type_of(&a), "i64");
    }
    #[test]
    fn i128_test() {
        let a: i128 = input::<i128>();
        assert_eq!(type_of(&a), "i128");
    }
    #[test]
    fn isize_test() {
        let a: isize = input::<isize>();
        assert_eq!(type_of(&a), "isize");
    }
    #[test]
    fn f32_test() {
        let a: f32 = input::<f32>();
        println!("{}", a);
        assert_eq!(type_of(&a), "f32");
    }
    #[test]
    fn f64_test() {
        let a: f64 = input::<f64>();
        assert_eq!(type_of(&a), "f64");
    }
    #[test]
    fn string_test() {
        let a: String = input::<String>();
        assert_eq!(type_of(&a), "alloc::string::String");
    }
}
