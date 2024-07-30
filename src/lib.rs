use std::error::Error;
/**
 * 导入标准库中的 fs 模块，这个模块提供了文件系统相关的功能，允许你执行诸如创建、打开、读取、写入、删除和移动文件等操作。
 * fs 模块中的一些常用功能包括：
 *      create_dir(path: &Path): 创建一个目录。
 *      create_dir_all(path: &Path): 创建一个目录及所有父目录。
 *      remove_dir(path: &Path): 删除一个空目录。
 *      remove_dir_all(path: &Path): 删除一个目录及其所有子目录和文件。
 *      remove_file(path: &Path): 删除一个文件。
 *      rename(from: &Path, to: &Path): 重命名或移动一个文件或目录。
 *      copy(from: &Path, to: &Path): 复制一个文件。
 *      read_to_string(path: &Path) -> io::Result<String>: 将文件内容读取为字符串。
 *      write(path: &Path, data: &[u8]) -> io::Result<usize>: 将数据写入文件。
 *
 */
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read_to_string 函数尝试读取文件的全部内容，并将内容转换为一个 String 类型的字符串
    // expect 是一个宏，用于处理 Result 类型的值。在这里，read_to_string 返回的 Result 要么包含一个 Ok(String) 成功情况，要么包含一个 Err(io::Error) 失败情况。
    // 如果函数成功读取了文件，expect 将返回 Ok 中的 String；但如果读取失败，expect 将触发一个 panic!
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{}", contents);
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
use std::env;
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // 第一个参数是程序名，由于无需使用，因此这里直接空调用一次
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
