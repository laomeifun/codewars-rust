// Codewars Kata: Returning Strings
// https://www.codewars.com/kata/returning-strings
// Difficulty: 8 kyu
//
// 创建一个函数，接收一个代表名字的参数，返回消息：
// "Hello, <name> how are you doing today?"

fn greet(name: &str) -> String {
    String::from(format!("Hello, {} how are you doing today?", name))
}

fn main() {
    println!("{}", greet("Ryan"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Ryan"), "Hello, Ryan how are you doing today?");
        assert_eq!(
            greet("Shingles"),
            "Hello, Shingles how are you doing today?"
        );
    }
}
