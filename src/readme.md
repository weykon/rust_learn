# Mutual Exclusion 

相互排斥

## mutex 是 Mutual Exclusion 缩写，
> 互斥器

互斥器以难以使用著称，因为你不得不记住：
1. 在使用数据之前尝试获取锁。
2. 处理完被互斥器所保护的数据之后，必须解锁数据，这样其他线程才能够获取锁。

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

```