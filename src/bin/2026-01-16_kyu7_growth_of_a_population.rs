// https://www.codewars.com/kata/563b662a59afc2b5120000c6/train/rust
// Growth of a Population (7 kyu)
//
// 题目描述：
// 在一个小镇，年初人口为 p0 = 1000。人口每年固定增长 2%，
// 此外每年有 50 名新居民迁入。需要多少年镇上人口才能达到或超过 p = 1200？
//
// 第一年末：1000 + 1000 * 0.02 + 50 => 1070 居民
// 第二年末：1070 + 1070 * 0.02 + 50 => 1141 居民（人口是整数）
// 第三年末：1141 + 1141 * 0.02 + 50 => 1213
// 需要 3 整年。
//
// 参数：
// p0 - 初始人口
// percent - 年增长百分比
// aug - 每年迁入（或迁出）的居民数
// p - 目标人口
//
// 返回达到或超过目标人口所需的整年数。
//
// 注意：
// - percent 参数需要转换为百分比（如 2 需转为 0.02）
// - 人口没有小数，每年末人口向下取整（如 252.8 取 252）
//
// 示例：
// nb_year(1500, 5, 100, 5000) -> 15
// nb_year(1500000, 2.5, 10000, 2000000) -> 10

fn nb_year(p0: i32, percent: f64, aug: i32, p: i32) -> i32 {
    let mut c: f64 = p0 as f64;
    let mut n = 0;
    while c < p as f64 {
        c = (c * percent / 100.0 + c + aug as f64).floor();
        n = n + 1;
    }
    n
}

fn main() {
    basic_tests();
    println!("All tests passed!");
}

fn basic_tests() {
    assert_eq!(nb_year(1500, 5.0, 100, 5000), 15);
    assert_eq!(nb_year(1500000, 2.5, 10000, 2000000), 10);
    assert_eq!(nb_year(1500000, 0.25, 1000, 2000000), 94);
    assert_eq!(nb_year(1500000, 0.25, -1000, 2000000), 151);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nb_year() {
        basic_tests();
    }
}
