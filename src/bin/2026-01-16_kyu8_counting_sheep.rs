// https://www.codewars.com/kata/54edbc7200b811e956000556/train/rust
// Counting sheep... (8 kyu)
//
// 题目描述：
// 考虑一个羊的数组/列表，其中一些羊可能从它们的位置上消失了。
// 我们需要一个函数来计算数组中存在的羊的数量（true 表示存在）。
//
// 例如：
// &[true,  true,  true,  false,
//   true,  true,  true,  true ,
//   true,  false, true,  false,
//   true,  false, false, true ,
//   true,  true,  true,  true ,
//   false, false, true,  true]
//
// 正确答案是 17。
//
// 提示：不要忘记检查 null/undefined 等无效值

fn count_sheep(s: &[bool]) -> u8 {
    s.iter().filter(|&&x| x).count() as u8
}

fn main() {
    basic_tests();
    println!("All tests passed!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(count_sheep(&[false]), 0);
        assert_eq!(count_sheep(&[true]), 1);
        assert_eq!(count_sheep(&[true, false]), 1);

        assert_eq!(
            count_sheep(&[
                true, true, true, false, true, true, true, true, true, false, true, false, true,
                false, false, true, true, true, true, true, false, false, true, true
            ]),
            17
        );
    }
}
