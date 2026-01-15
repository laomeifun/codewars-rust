// https://www.codewars.com/kata/57a0885cbb9944e24c00008e/train/rust
// Remove exclamation marks (8 kyu)
//
// 题目描述：
// 编写函数 remove_exclamation_marks，从给定字符串中移除所有感叹号。
//
// 示例：
// "Hello World!" -> "Hello World"
// "Hello!!! World!" -> "Hello World"

fn remove_exclamation_marks(s: &str) -> String {
    todo!()
}

fn main() {
    basic_tests();
    println!("All tests passed!");
}

fn basic_tests() {
    assert_eq!(remove_exclamation_marks("Hello World!"), "Hello World");
    assert_eq!(remove_exclamation_marks("Hello World!!!"), "Hello World");
    assert_eq!(remove_exclamation_marks("Hi! Hello!"), "Hi Hello");
    assert_eq!(remove_exclamation_marks(""), "");
    assert_eq!(remove_exclamation_marks("!"), "");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_exclamation_marks() {
        basic_tests();
    }
}
