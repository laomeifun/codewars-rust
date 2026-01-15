// https://www.codewars.com/kata/55d24f55d7dd296eb9000030/train/rust
// Grasshopper - Summation (8 kyu)
//
// 题目描述：
// 编写一个程序，计算从 1 到 num（包含两端）的所有数字之和。
// 数字始终是大于 0 的正整数。
// 函数只需返回结果。
//
// 例如（输入 -> 输出）：
// 2 -> 3 (1 + 2)
// 8 -> 36 (1 + 2 + 3 + 4 + 5 + 6 + 7 + 8)

fn summation(n: i32) -> i32 {
    (1..=n).sum()
}

fn main() {
    basic_tests();
    println!("All tests passed!");
}

fn basic_tests() {
    assert_eq!(summation(1), 1);
    assert_eq!(summation(2), 3);
    assert_eq!(summation(8), 36);
    assert_eq!(summation(10), 55);
    assert_eq!(summation(100), 5050);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summation() {
        basic_tests();
    }
}
