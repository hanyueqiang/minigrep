use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read_to_string 函数尝试读取文件的全部内容，并将内容转换为一个 String 类型的字符串
    // expect 是一个宏，用于处理 Result 类型的值。在这里，read_to_string 返回的 Result 要么包含一个 Ok(String) 成功情况，要么包含一个 Err(io::Error) 失败情况。
    // 如果函数成功读取了文件，expect 将返回 Ok 中的 String；但如果读取失败，expect 将触发一个 panic!
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{}", contents);
    Ok(())
}
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
