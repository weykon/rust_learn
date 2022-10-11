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
    let a = "1";
    {
        let b = "2";
        compare(b, a);
        compare(a.clone(), b.clone());
    }
}
