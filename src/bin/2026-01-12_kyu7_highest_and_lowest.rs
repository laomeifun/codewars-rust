// https://www.codewars.com/kata/554b4ac871d6813a03000035/train/rust
// Highest and Lowest (7 kyu)
//
// 题目描述：
// 给定一个由空格分隔的数字字符串，需要返回其中的最高值和最低值。
//
// 示例：
// high_and_low("1 2 3 4 5")   => "5 1"
// high_and_low("1 2 -3 4 5")  => "5 -3"
// high_and_low("1 9 3 4 -5")  => "9 -5"
//
// 注意事项：
// - 所有数字都是有效的 Int32，无需验证
// - 输入字符串中至少包含一个数字
// - 输出字符串必须是两个数字，用单个空格分隔，最高值在前

fn high_and_low(numbers: &str) -> String {
    let nums: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let max = nums.iter().max().unwrap();
    let min = nums.iter().min().unwrap();
    format!("{} {}", max, min)
}

fn main() {
    fixed_tests();
    println!("All tests passed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"), "42 -9");
        assert_eq!(high_and_low("1 2 3"), "3 1");
    }
}

fn fixed_tests() {
    assert_eq!(high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"), "42 -9");
    assert_eq!(high_and_low("1 2 3"), "3 1");
}
