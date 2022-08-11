智能指针跟引用的不同

手写一些Box这个标准库来看看里面有什么内容


## 第一个： Deref

## 第二个： Drop
    用法比较简单，相当于在析构前的代码

### 还有 std::mem::drop 提早丢弃值
 原本是不允许显式调用 drop 的，这里考虑是 main 的结尾是自动调用drop，这会导致 double free错误，因为Rust会尝试清理相同的值两次。
 这里使用 std:mem::drop 和 Drop trait 不同