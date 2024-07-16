use minigrep::Config;
/**
 * 引入标准库中的 env 模块，这个模块提供了与环境相关的功能，包括但不限于读取环境变量和命令行参数。
 * std::env 模块包含了以下几个主要功能：
 * 1、命令行参数：
 *      args()：返回一个迭代器，它包含了程序启动时从命令行传入的所有参数。第一个元素通常是程序的路径，后续元素则是用户提供的参数。
 * 2、环境变量：
 *      var(name: &str)：获取指定名称的环境变量的值，如果环境变量不存在则会触发一个错误。
 *      var_os(name: &OsStr)：类似于 var，但是返回的是 OsString 类型，这可以避免字符编码问题。
 *      vars()：返回一个迭代器，遍历所有环境变量及其对应的值。
 *      remove_var(name: &str)：移除指定名称的环境变量。
 *      set_var(name: &str, val: &str)：设置或修改环境变量。
 * 3、其他环境信息：
 *      current_exe()：返回一个表示当前执行文件的路径。
 *      current_dir()：返回当前工作目录的路径。
 *      temp_dir()：返回系统临时目录的路径。
 */
use std::env;
use std::process;
fn main() {
    // 调用了 std::env 模块中的 args 函数，它返回一个迭代器，其中包含了所有从命令行传递给程序的参数
    // .collect() 是一个迭代器方法，用于将迭代器中的元素收集到一个集合中。
    let args: Vec<String> = env::args().collect();
    // dbg! 是一个宏，通常用于调试目的。它接受一个表达式作为参数，并打印出该表达式的值以及其类型。这在调试时非常有用，因为它不仅显示了变量的值，还显示了变量的数据类型
    // dbg!(args);
    // 对 build 返回的 `Result` 进行处理
    // unwrap_or_else 是定义在 Result<T,E> 上的常用方法，如果 Result 是 Ok，那该方法就类似 unwrap：返回 Ok 内部的值；如果是 Err，就调用闭包中的自定义代码对错误进行进一步处理
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {} ", config.query);
    println!("In file {} ", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
