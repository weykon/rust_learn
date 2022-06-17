* The "mod" keyword declares a submodule
-> mod关键字将其声明一个子模块
如果 mod my_module; 
那么编译器会去 my_module.rs 或者 my_module/mod.rs 里面找

## 第二课

用mod.rs

[学习地址](https://www.sheshbabu.com/posts/rust-module-system/)

## super、self

```rust
fn func(){}
mod my{
    fn func(){}
    fn test(){
        self::func();   // my::func 
        func():         // my::func

        super::func();  // (outside the `my` mod) func
    }
}
```

# cfg! 和 #[cfg]

跟C预处理程序有相似，rust的宏定义
cfg 用来生成文档是做文档内代码测试的，也就是我们运行rustdoc --test 或者cargo test运行到doc部分的时候，设置了这个宏内的代码就会被运行。
[knowledge](https://rustcc.cn/article?id=40c5456f-824f-427d-a9c9-485cf3de7ef3)

# lib.rs 和 main.rs
* 当您的项目有一个src/lib.rs文件时，它就是一个库，可以被其他板条箱调用。
* 当您的项目有一个src/main.rs文件时，它就被构建到一个可执行文件中。
您的项目可以同时具有这两种功能，从而简化了将应用程序的功能公开给其他机箱的过程。
