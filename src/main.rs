fn main() {
    let name = format!("fellow RUstaceans");
    helper(&name);
    helper(&name);
    let r = &name;
    helper(r);
    helper(r);

    test_mut_situation();

    test_use_a_block_siutuation();
}

fn helper(name: &String) {
    println!("{}", name);
}

fn test_mut_situation() {
    let mut name = format!("fellow RUstaceans");

    // 并不是说就不能用可变，具体是，是哪里开始借用，when

    name.push('x'); // 这里还没有借用

    helper(&name);

    let r = &name;

    // name.push('x');    // 这里会报错了， 因为，他是mut的，
    // 在上面已经声明 r = &name, 这个push操作很可能就会将原来的数据变了
    // 下面的helper就可能找不到; 所以在调用的顺序中，有这种思维保留着。

    helper(r); // 这里开始调用，
}

fn test_use_a_block_siutuation() {
    let mut name = format!("fellow RUstaceans");

    name.push('x');

    {
        let r = &name;
        helper(r);
        helper(r);
    } // <-- borrow ends here 

    // r  // out of scope
    name.push('x');
}
