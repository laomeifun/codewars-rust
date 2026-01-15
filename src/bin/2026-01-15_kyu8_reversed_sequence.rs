// https://www.codewars.com/kata/5a00e05cc374cb34d100000d/train/rust
// Reversed sequence (8 kyu)
//
// 构建一个函数，返回从 n 到 1 的整数数组，其中 n>0。
//
// 示例：n=5 --> [5,4,3,2,1]

fn reverse_seq(n: u32) -> Vec<u32> {
    (1..=n).rev().collect()
}

fn main() {
    tests::sample_test();
    println!("All tests passed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(reverse_seq(5), [5, 4, 3, 2, 1].to_vec());
    }
}
