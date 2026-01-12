// https://www.codewars.com/kata/5bb904724c47249b10000131
// Total amount of points (8 kyu)
//
// 题目描述:
// 足球队完成了锦标赛，比赛结果记录在字符串集合中。
// 每场比赛用 "x:y" 格式表示，x 是我方得分，y 是对方得分。
//
// 示例: ["3:1", "2:2", "0:1", ...]
//
// 积分规则:
//   - x > y: 3 分 (胜)
//   - x < y: 0 分 (负)
//   - x = y: 1 分 (平)
//
// 约束:
//   - 固定 10 场比赛
//   - 0 <= x <= 4
//   - 0 <= y <= 4

use std::cmp::Ordering;

fn points(games: &[String]) -> u32 {
    games
        .iter()
        .map(|c| {
            let parts: Vec<&str> = c.split(':').collect();
            let x: u32 = parts[0].parse().unwrap();
            let y: u32 = parts[1].parse().unwrap();
            match x.cmp(&y) {
                Ordering::Greater => 3, // 胜
                Ordering::Equal => 1,   // 平
                Ordering::Less => 0,    // 负
            }
        })
        .sum()
}

fn main() {
    // 测试入口
    println!("Running tests...");
    fixed_tests();
    println!("All tests passed!");
}

const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

fn do_fixed_test(e: &[&str], expected: u32) {
    let a = &e.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    assert_eq!(points(a), expected, "{ERR_MSG} with games = {a:?}")
}

fn fixed_tests() {
    // 全胜: 10 * 3 = 30
    do_fixed_test(
        &[
            "1:0", "2:0", "3:0", "4:0", "2:1", "3:1", "4:1", "3:2", "4:2", "4:3",
        ],
        30,
    );
    // 全平: 10 * 1 = 10
    do_fixed_test(
        &[
            "1:1", "2:2", "3:3", "4:4", "2:2", "3:3", "4:4", "3:3", "4:4", "4:4",
        ],
        10,
    );
    // 全负: 10 * 0 = 0
    do_fixed_test(
        &[
            "0:1", "0:2", "0:3", "0:4", "1:2", "1:3", "1:4", "2:3", "2:4", "3:4",
        ],
        0,
    );
}
