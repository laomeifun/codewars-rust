// 工具：自动生成 Codewars kata 文件
// 用法: cargo run --bin new_kata -- <kata_url_or_slug>
// 例如: cargo run --bin new_kata -- 50654ddff44f800200000004
//       cargo run --bin new_kata -- https://www.codewars.com/kata/50654ddff44f800200000004
//
// 获取代码模板和测试（可选）:
//   1. 在浏览器登录 Codewars
//   2. 打开开发者工具 -> Application -> Cookies
//   3. 复制 _session_id 的值
//   4. 创建 .env 文件: CODEWARS_SESSION=<session_id>
//      或设置环境变量: set CODEWARS_SESSION=<session_id>
//   5. 运行本工具

use std::env;
use std::fs;
use std::io::{BufRead, Write};
use std::process::Command;

fn main() {
    // 加载 .env 文件
    load_env_file();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: cargo run --bin new_kata -- <kata_url_or_slug>");
        eprintln!("例如: cargo run --bin new_kata -- 50654ddff44f800200000004");
        std::process::exit(1);
    }

    // 从 URL 或 slug 提取 kata ID
    let input = &args[1];
    let slug = extract_slug(input);

    println!("正在获取 kata 信息: {}", slug);

    // 调用 Codewars API
    let api_url = format!("https://www.codewars.com/api/v1/code-challenges/{}", slug);

    let output = Command::new("curl")
        .args(["-s", &api_url])
        .output()
        .expect("无法执行 curl");

    let json_str = String::from_utf8_lossy(&output.stdout);

    // 简单解析 JSON (不引入依赖)
    let name = extract_json_field(&json_str, "name").unwrap_or_else(|| "unknown".to_string());
    let slug_from_api = extract_json_field(&json_str, "slug").unwrap_or_else(|| slug.clone());
    let rank = extract_rank(&json_str).unwrap_or(8);
    let description = extract_description(&json_str).unwrap_or_default();
    let url = format!("https://www.codewars.com/kata/{}", slug_from_api);

    // 尝试获取代码模板和测试（需要 session）
    let session = env::var("CODEWARS_SESSION").ok();
    let (solution_code, test_code) = if let Some(ref session_id) = session {
        println!("正在获取代码模板...");
        fetch_code_template(&slug, "rust", session_id)
    } else {
        println!("提示: 在 .env 文件中设置 CODEWARS_SESSION 可获取代码模板和测试");
        (None, None)
    };

    // 生成文件名
    let today = get_today();
    let safe_name = name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .trim_matches('_')
        .replace("__", "_")
        .to_string();

    let filename = format!("src/bin/{}_kyu{}_{}.rs", today, rank, safe_name);

    // 生成文件内容
    let desc_comments = format_description(&description);

    // 使用获取的代码模板，或使用默认模板
    let solution_part = solution_code.unwrap_or_else(|| "fn solution() {\n    todo!()\n}".to_string());
    let test_part = test_code.unwrap_or_else(|| "    #[test]\n    fn basic_test() {\n        // todo: 添加测试\n    }".to_string());

    let content = format!(r#"// Codewars Kata: {}
// {}
// Difficulty: {} kyu
//
{}

{}

fn main() {{
    println!("{{:?}}", solution());
}}

#[cfg(test)]
mod tests {{
    use super::*;

{}
}}
"#, name, url, rank, desc_comments, solution_part, test_part);

    // 写入文件
    let mut file = fs::File::create(&filename).expect("无法创建文件");
    file.write_all(content.as_bytes()).expect("无法写入文件");

    println!("✓ 已创建: {}", filename);
    println!("  题目: {} ({}kyu)", name, rank);
    println!("  链接: {}", url);
}

fn load_env_file() {
    // 尝试加载 .env 文件，支持多个位置
    let env_paths = [".env", "codewars.env"];

    for path in env_paths {
        if let Ok(file) = fs::File::open(path) {
            let reader = std::io::BufReader::new(file);
            for line in reader.lines().map_while(Result::ok) {
                let line = line.trim();
                // 跳过空行和注释
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                // 解析 KEY=VALUE 格式
                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim();
                    let value = value.trim().trim_matches('"').trim_matches('\'');
                    // 只在环境变量未设置时才设置
                    if env::var(key).is_err() {
                        // SAFETY: 单线程环境，不会有并发访问
                        unsafe { env::set_var(key, value); }
                    }
                }
            }
            break;
        }
    }
}

fn extract_slug(input: &str) -> String {
    // 从 URL 提取 slug，或直接使用输入作为 slug
    // 支持格式:
    //   - 57eae20f5500ad98e50002c5
    //   - https://www.codewars.com/kata/57eae20f5500ad98e50002c5
    //   - https://www.codewars.com/kata/57eae20f5500ad98e50002c5/train/rust
    //   - https://www.codewars.com/kata/some-kata-name/solutions/rust
    if let Some(kata_pos) = input.find("/kata/") {
        let after_kata = &input[kata_pos + 6..];
        // 取 /kata/ 后的第一段路径
        after_kata.split('/').next().unwrap_or(after_kata).to_string()
    } else {
        input.to_string()
    }
}

fn extract_json_field(json: &str, field: &str) -> Option<String> {
    let pattern = format!("\"{}\":\"", field);
    if let Some(start) = json.find(&pattern) {
        let rest = &json[start + pattern.len()..];
        if let Some(end) = rest.find('"') {
            return Some(rest[..end].to_string());
        }
    }
    None
}

fn extract_rank(json: &str) -> Option<u8> {
    // 查找 "rank":{"id":-8 这样的模式
    if let Some(start) = json.find("\"rank\":{\"id\":") {
        let rest = &json[start + 13..];
        if let Some(end) = rest.find(',') {
            let num_str = &rest[..end];
            if let Ok(n) = num_str.parse::<i8>() {
                return Some(n.unsigned_abs());
            }
        }
    }
    None
}

fn get_today() -> String {
    let output = Command::new("powershell")
        .args(["-Command", "Get-Date -Format 'yyyy-MM-dd'"])
        .output()
        .expect("无法获取日期");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn extract_description(json: &str) -> Option<String> {
    // 提取 "description":"..." 字段，处理转义字符
    let pattern = "\"description\":\"";
    if let Some(start) = json.find(pattern) {
        let rest = &json[start + pattern.len()..];
        let mut result = String::new();
        let mut chars = rest.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '\\' => {
                    if let Some(&next) = chars.peek() {
                        chars.next();
                        match next {
                            'n' => result.push('\n'),
                            'r' => result.push('\r'),
                            't' => result.push('\t'),
                            '"' => result.push('"'),
                            '\\' => result.push('\\'),
                            _ => {
                                result.push('\\');
                                result.push(next);
                            }
                        }
                    }
                }
                '"' => break,
                _ => result.push(c),
            }
        }
        if !result.is_empty() {
            return Some(result);
        }
    }
    None
}

fn format_description(desc: &str) -> String {
    // 将描述转换为注释格式，每行加 // 前缀
    desc.lines()
        .map(|line| format!("// {}", line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn fetch_code_template(slug: &str, language: &str, session_id: &str) -> (Option<String>, Option<String>) {
    // 首先尝试获取训练页面以启动 session
    let train_url = format!("https://www.codewars.com/kata/{}/train/{}", slug, language);
    let cookie = format!("_session_id={}", session_id);

    // 访问训练页面
    let _ = Command::new("curl")
        .args(["-s", "-b", &cookie, &train_url])
        .output();

    // 然后尝试获取 session 数据
    let session_url = format!("https://www.codewars.com/kata/{}/{}/session", slug, language);

    let output = Command::new("curl")
        .args([
            "-s",
            "-b", &cookie,
            "-H", "Accept: application/json",
            "-H", "X-Requested-With: XMLHttpRequest",
            &session_url
        ])
        .output()
        .expect("无法执行 curl");

    let json_str = String::from_utf8_lossy(&output.stdout);

    // 解析代码模板
    let solution = extract_code_field(&json_str, "setup")
        .or_else(|| extract_code_field(&json_str, "code"));

    // 解析测试代码
    let tests = extract_code_field(&json_str, "exampleFixture")
        .or_else(|| extract_code_field(&json_str, "fixture"));

    // 格式化测试代码（添加缩进）
    let formatted_tests = tests.map(|t| {
        t.lines()
            .map(|line| format!("    {}", line))
            .collect::<Vec<_>>()
            .join("\n")
    });

    if solution.is_some() || formatted_tests.is_some() {
        println!("  ✓ 成功获取代码模板");
    } else {
        println!("  ✗ 无法获取代码模板（session 可能已过期）");
    }

    (solution, formatted_tests)
}

fn extract_code_field(json: &str, field: &str) -> Option<String> {
    // 提取代码字段，处理转义字符
    let pattern = format!("\"{}\":\"", field);
    if let Some(start) = json.find(&pattern) {
        let rest = &json[start + pattern.len()..];
        let mut result = String::new();
        let mut chars = rest.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '\\' => {
                    if let Some(&next) = chars.peek() {
                        chars.next();
                        match next {
                            'n' => result.push('\n'),
                            'r' => {},
                            't' => result.push('\t'),
                            '"' => result.push('"'),
                            '\\' => result.push('\\'),
                            'u' => {
                                // 处理 \uXXXX unicode 转义
                                let hex: String = chars.by_ref().take(4).collect();
                                if let Ok(code) = u32::from_str_radix(&hex, 16) {
                                    if let Some(ch) = char::from_u32(code) {
                                        result.push(ch);
                                    }
                                }
                            }
                            _ => {
                                result.push('\\');
                                result.push(next);
                            }
                        }
                    }
                }
                '"' => break,
                _ => result.push(c),
            }
        }
        if !result.is_empty() {
            return Some(result);
        }
    }
    None
}
