fn main() {
    let path = "/tmp/dat";
    println!("{}", read_file(path));
}

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}
// 这段代码中，就如果输入的path地址有误，读取不了文件，程序竟然会出错崩溃
// 这里编译就会提示使用 Result::unwrap() 来解决
// 大量代码中很难避免 unwrap() 的出现

// 在rust中尽量
fn fixProbablyNice(path: &str){
    let path = "/tmp/dat";  //文件路径
    match read_file1(path) { //判断方法结果
        Ok(file) => { println!("{}", file) } //OK 代表读取到文件内容，正确打印文件内容
        Err(e) => { println!("{} {}", path, e) } //Err代表结果不存在，打印错误结果
    }
}
fn read_file1(path: &str) -> Result<String,std::io::Error> { //Result作为结果返回值
    std::fs::read_to_string(path) //读取文件内容
}