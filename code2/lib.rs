extern crate std;
// 默认情况下，编译器extern crate std;会在 crate 根目录的开头插入（板条箱根目录是您传递给的文件rustc）。
// 该语句的作用是将名称添加std到 crate 的根命名空间，并将其与包含stdcrate 的公共内容的模块相关联。

// 但是，在子模块中，std不会自动添加到模块的命名空间中。
// 这就是编译器无法解析模块中std（或以 开头的任何内容std::）的原因。

pub fn any_func(){

}