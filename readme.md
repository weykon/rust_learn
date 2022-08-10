## 两类: 

1. 描述型
2. 程序型  

### **描述型**

     - example: 
    
    

```rust
    // use macro_rules! <name of macro>{<Body>}
    macro_rules! add{
     // macth like arm for macro
        ($a:expr,$b:expr)=>{
     // macro expand to this code
            {
    // $a and $b will be templated using the value/ variable provided to macro
                $a+$b
            }
        }
    }

    fn main(){
     // call to macro, $a=1 and $b=2
        add!(1,2);
    }
```

    

```rust
    macro_rules! add{
    // first arm match add!(1,2), add!(2,3) etc
        ($a:expr,$b:expr)=>{
            {
                $a+$b
            }
        };
    // Second arm macth add!(1), add!(2) etc
        ($a:expr)=>{
            {
                $a
            }
        }
    }

    fn main(){
    // call the macro
        let x=0;
        add!(1,2);
        add!(x);
    }   
```

```rust
    macro_rules! add_as{
    // using a ty token type for macthing   datatypes passed to maccro
        ($a:expr,$b:expr,$typ:ty)=>{
            $a as $typ + $b as $typ
        }
    }

    fn main(){
        println!("{}",add_as!(0,2,u8));
    }
```

* item — an item, like a function, struct, module, etc.
* block — a block (i.e. a block of statements and/or an expression, surrounded by braces)
* stmt — a statement
* pat — a pattern
* expr — an expression
* ty — a type
* ident — an identifier
* path — a path (e.g., foo, ::std::mem::replace, transmute::<_, int>, …)
* meta — a meta item; the things that go inside #[...] and #![...] attributes
* tt — a single token tree
* vis — a possibly empty Visibility qualifier

这些内置的关键字

* 还有像正则的东西，来应对多种复杂情况，有“ + ”, “ * ”.

```rust
    macro_rules! add_as{
        ($($a:expr),*)=>{
            { 
                0
                $(+$a)*
            }
        }
    }

    fn main(){
        println!("{}",add_as!(1,2,3,4)); // =>  println!("{}",{0+1+2+3+4})
    }
```

```rust
    macro_rules! add{
     // first arm in case of single argument and    last remaining variable/number
        ($a:expr)=>{
            $a
        };
    // second arm in case of two arument are    passed and stop recursion in case of odd   number ofarguments
        ($a:expr,$b:expr)=>{
            {
                $a+$b
            }
        };
    // add the number and the result of remaining   arguments 
        ($a:expr,$($b:tt)*)=>{
           {
               $a+add!($($b)*)
           }
        }
    }

    fn main(){
        println!("{}",add!(1,2,3,4));
    }
```


### **程序型**
