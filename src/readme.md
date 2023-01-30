用包来声明macro derive
keyword attribute:  proc_macro_derive


ast 是一下结构的数据
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
那么直接在 ast.ident 就去到了这个结构体的名字了。


  
#[cfg]之外的另一个重要的属性是#[derive]，它用于生成特定的方法，以方便在Rust类型上操作，如序列化，反序列化，比较，哈希，复制等。

还有：

#[macro_use]：允许从外部宏包导入宏。
#[allow(...)]：允许编译器在代码编译时忽略特定的警告。
#[no_mangle]：告诉编译器不要对函数的名称进行重整。
#[repr(...)]：指定如何表示结构体或枚举类型。