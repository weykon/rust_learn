这里分支了主要是因为这个

```rust
enum List{
    Cons(i32,Box<List>),
    Nil
} 
```

在rustlings中，需要用这个枚举来实例一下，发现自己不会了，
有些忘记了对枚举体的操作

```rust
    List::Cons(0,List::Nil)
```