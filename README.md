# [Below content belong to Tokens concept in Rust](https://doc.rust-lang.org/reference/tokens.html#tokens)




[Raw string literals](https://doc.rust-lang.org/reference/tokens.html#raw-string-literals)
What do r / r# do ? 

```
"foo"; r"foo";                     // foo
"\"foo\""; r#""foo""#;             // "foo"

"foo #\"# bar";
r##"foo #"# bar"##;                // foo #"# bar

"\x52"; "R"; r"R";                 // R
"\\x52"; r"\x52";                  // \x52
```


// TODO: b'