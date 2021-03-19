use std::fmt::Debug;
use std::fmt::Display;
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn u8_test() {
        let a: u8 = input::<u8>();
        println!("{}", a);
    }
    #[test]
    fn u16_test() {
        let a: u16 = input::<u16>();
        println!("{}", a);
    }
    #[test]
    fn u32_test() {
        let a: u32 = input::<u32>();
        println!("{}", a);
    }
    #[test]
    fn u128_test() {
        let a: u128 = input::<u128>();
        println!("{}", a);
    }
    #[test]
    fn usize_test() {
        let a: usize = input::<usize>();
        println!("{}", a);
    }
    #[test]
    fn i8_test() {
        let a: i8 = input::<i8>();
        println!("{}", a);
    }
    #[test]
    fn i16_test() {
        let a: i16 = input::<i16>();
        println!("{}", a);
    }
    #[test]
    fn i32_test() {
        let a: i32 = input::<i32>();
        println!("{}", a);
    }
    #[test]
    fn i64_test() {
        let a: i64 = input::<i64>();
        println!("{}", a);
    }
    #[test]
    fn i128_test() {
        let a: i128 = input::<i128>();
        println!("{}", a);
    }
    #[test]
    fn isize_test() {
        let a: isize = input::<isize>();
        println!("{}", a);
    }
    #[test]
    fn f32_test() {
        let a: f32 = input::<f32>();
        println!("{}", a);
    }
    #[test]
    fn f64_test() {
        let a: f64 = input::<f64>();
        println!("{}", a);
    }
    #[test]
    fn string_test() {
        let a: String = input::<String>();
        println!("{}", a);
    }
}
