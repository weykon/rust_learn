* The "mod" keyword declares a submodule
-> mod关键字将其声明一个子模块
如果 mod my_module;
那么编译器会去 my_module.rs 或者 my_module/mod.rs 里面找

## 第二课
用mod.rs

## 同时在函数内外使用引用导致的重复借用错误
https://course.rs/compiler/fight-with-compiler/borrowing/ref-exist-in-out-fn.html


在日常使用中，基本都会遇到重复借用的问题，要透彻所有的问题，需要专项的例子帮助理解
