// https://www.codewars.com/kata/544675c6f971f7399a000e79/train/rust
// Convert a String to a Number! (8 kyu)
//
// 题目描述：
// 我们需要一个能将字符串转换为数字的函数。你知道有哪些方法可以实现吗？
//
// 注意：不用担心，所有输入都是字符串，并且每个字符串都是一个完全有效的整数表示。
//
// 示例：
// "1234" --> 1234
// "605"  --> 605
// "1405" --> 1405
// "-7"   --> -7

fn string_to_number(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn main() {
    tests::returns_expected();
    tests::works_on_random();
    println!("All tests passed!");
}

#[cfg(test)]
mod tests {
    use super::string_to_number;
    use rand::prelude::*;

    #[test]
    pub fn returns_expected() {
        assert_eq!(string_to_number("1234"), 1234);
        assert_eq!(string_to_number("605"), 605);
        assert_eq!(string_to_number("1405"), 1405);
        assert_eq!(string_to_number("-7"), -7);
    }

    #[test]
    pub fn works_on_random() {
        let mut rng = thread_rng();
        for _ in 0..5 {
            let num: i32 = rng.gen();
            let input = num.to_string();
            assert_eq!(string_to_number(&input), num);
        }
    }
}
