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