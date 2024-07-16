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
fn main() {
    // 调用了 std::env 模块中的 args 函数，它返回一个迭代器，其中包含了所有从命令行传递给程序的参数
    // .collect() 是一个迭代器方法，用于将迭代器中的元素收集到一个集合中。
    let args: Vec<String> = env::args().collect();
    // dbg! 是一个宏，通常用于调试目的。它接受一个表达式作为参数，并打印出该表达式的值以及其类型。这在调试时非常有用，因为它不仅显示了变量的值，还显示了变量的数据类型
    // dbg!(args);
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {} ", query);
    println!("In file {} ", file_path);
    // read_to_string 函数尝试读取文件的全部内容，并将内容转换为一个 String 类型的字符串
    // expect 是一个宏，用于处理 Result 类型的值。在这里，read_to_string 返回的 Result 要么包含一个 Ok(String) 成功情况，要么包含一个 Err(io::Error) 失败情况。
    // 如果函数成功读取了文件，expect 将返回 Ok 中的 String；但如果读取失败，expect 将触发一个 panic!
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("With text:\n{}", contents);
}
