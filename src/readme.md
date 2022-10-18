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