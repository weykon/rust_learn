// 需要注明 生命周期 的情况下，是这样的
// 比如

fn main() {}

fn compare<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str
where
    'b: 'a,
{
    if s2.len() > s1.len() {
        s2 // 在这个函数中，有可能会返回 s2, 而 s2 属于 'b 的 ， 现在是标注好 'a : 'b ，意思是， 'a 在 'b 生命周期外层，包括 'b
    } else {
        s1
    }
}

// example new one 比如在同一个函数中的不同生命周期
fn myMultiLife() {
    let a = "112312312";
    let aa: String = String::from("123123");
    {
        // 因为这里是基本类型，实质上不clone也是可以的，不过只是演示.
        // 但是，这些基本类型，实质上他们隐式复制的，直接copy有两份。
        let b = "2";
        compare(b, a);
        compare(a.clone(), b.clone());
    }
    containString(aa);
    // containString(aa); 比如这里的String，使用两次，这里就会报错的
}
fn containString(string: String) -> String {
    string
}
