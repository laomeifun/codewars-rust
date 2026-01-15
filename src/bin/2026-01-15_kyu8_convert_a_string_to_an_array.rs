// Codewars Kata: Convert a string to an array
// https://www.codewars.com/kata/convert-a-string-to-an-array
// Difficulty: 8 kyu
//
// 编写一个函数，将字符串按空格分割并转换为单词数组。
//
// 示例 (输入 ==> 输出):
// "Robin Singh" ==> ["Robin", "Singh"]
// "I love arrays they are my favorite" ==> ["I", "love", "arrays", "they", "are", "my", "favorite"]

fn string_to_array(s: &str) -> Vec<String> {
    todo!()
}

fn main() {
    println!("{:?}", string_to_array("Robin Singh"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_array() {
        assert_eq!(
            string_to_array("Robin Singh"),
            vec!["Robin", "Singh"]
        );
        assert_eq!(
            string_to_array("I love arrays they are my favorite"),
            vec!["I", "love", "arrays", "they", "are", "my", "favorite"]
        );
    }
}
