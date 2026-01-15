// https://www.codewars.com/kata/558fc85d8fd1938afb000014/train/rust
// Sum of two lowest positive integers (7 kyu)
//
// 题目描述：
// 创建一个函数，返回给定数组中两个最小正整数的和。
// 数组至少包含 4 个正整数。不会传入浮点数或非正整数。
//
// 例如：
// [19, 5, 42, 2, 77] 应该返回 7 (2 + 5)
// [10, 343445353, 3453445, 3453545353453] 应该返回 3453455

fn sum_two_smallest_numbers(numbers: &[u64]) -> u64 {
    {
        let mut v = numbers.to_vec();
        v.sort();
        v[0] + v[1]
    }
}

fn main() {
    basic_tests();
    println!("All tests passed!");
}

fn basic_tests() {
    assert_eq!(sum_two_smallest_numbers(&[5, 8, 12, 19, 22]), 13);
    assert_eq!(sum_two_smallest_numbers(&[15, 28, 4, 2, 43]), 6);
    assert_eq!(sum_two_smallest_numbers(&[3, 87, 45, 12, 7]), 10);
    assert_eq!(sum_two_smallest_numbers(&[23, 71, 33, 82, 1]), 24);
    assert_eq!(sum_two_smallest_numbers(&[52, 76, 14, 12, 4]), 16);
    assert_eq!(sum_two_smallest_numbers(&[19, 5, 42, 2, 77]), 7);
    assert_eq!(
        sum_two_smallest_numbers(&[10, 343445353, 3453445, 3453545353453]),
        3453455
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_two_smallest_numbers() {
        basic_tests();
    }
}
