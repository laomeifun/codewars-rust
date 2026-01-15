// https://www.codewars.com/kata/5949481f86420f59480000e7/train/rust
// Odd or Even? (7 kyu)
//
// 给定一个整数列表，判断其元素之和是奇数还是偶数。
//
// 以字符串形式返回答案："odd" 或 "even"。
//
// 如果输入数组为空，将其视为 [0]（包含零的数组）。
//
// 示例：
// 输入: [0]       输出: "even"
// 输入: [0, 1, 4] 输出: "odd"
// 输入: [0, -1, -5] 输出: "even"

fn odd_or_even(n: Vec<i32>) -> String {
    match n.iter().sum::<i32>() % 2 {
        0 => "even".to_string(),
        _ => "odd".to_string(),
    }
}

fn main() {
    tests::test_empty_array();
    tests::test_single_even();
    tests::test_single_odd();
    tests::test_even();
    tests::test_odd();
    tests::test_negative_even();
    tests::test_negative_odd();
    println!("All tests passed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        assert_eq!(odd_or_even(vec![]), "even");
    }
    #[test]
    fn test_single_even() {
        assert_eq!(odd_or_even(vec![0]), "even");
    }
    #[test]
    fn test_single_odd() {
        assert_eq!(odd_or_even(vec![1]), "odd");
    }
    #[test]
    fn test_even() {
        assert_eq!(odd_or_even(vec![0, 1, 5]), "even");
    }
    #[test]
    fn test_odd() {
        assert_eq!(odd_or_even(vec![0, 1, 4]), "odd");
    }
    #[test]
    fn test_negative_even() {
        assert_eq!(odd_or_even(vec![0, -1, -5]), "even");
    }
    #[test]
    fn test_negative_odd() {
        assert_eq!(odd_or_even(vec![0, -1, 2]), "odd");
    }
}
