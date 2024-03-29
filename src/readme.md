<<<<<<< HEAD
# lesson0 & lesson_1

```rust
 let a = 0; // 不可变
 let mut b = 0; // 可变
```

* String 是标准库中的字符串类型，使用UTF-8.
+String::new 的new函数在rust中叫做关联函数，也就是平时说的静态函数.

* println {} 是一个占位符，后面的参数值插入自己预留的特定位置。（按顺序的）
# lesson_2

类型：i32（32位整型）、u32（无符号整型）

? 重复使用变量名？
* rust允许使用同名的新变量，这个特性通常使用在转换值类型的场景。（隐藏机制）

* trim() 抹除了用户输入的回车键

* match 表达式由多个分支（arm）组成，每个分支是包含一个用于匹配的模式（pattern）
# lesson_3

## loop 循环 咯～  

* 关键字 _break_

```rust
        let mut guess = String::new(); // 这条如果放在loop外面就不能正常运行了
```

估计是作用域相关的内容。

# lesson_4

let出来的变量没有mut是这个值不可变，但是rust还是有常量的概念，使用const来声明常量。
[这里有let、static、const的区别解释，易懂](https://rustcc.cn/article?id=d3954670-a58a-427d-9c0c-6666051f5cc7)

## 隐藏

    刚才的同名变量，第二次赋值的guess将第一次赋值的隐藏了，（shadow）了。随后使用这个名称，是指向了第二个变量。
    隐藏机制不同于mut声明，这里可以通过隐藏机制。不断重新给变量赋值，执行一系列的操作，且能保持自己的不可变性。

* 比如： 
```rust 

    let spaces = "  ";
    let spaces = spaces.len(); // 是没有问题；
    // 但是
    let mut spaces = "  ";
    spaces = spaces.len(); //就不行了

```

# lesson_5 
数据结构分为两种： 
- 1.标量类型
    整数、浮点、布尔、字符
- 2.复合类型
    元组、数组

# lesson_6
有趣： let a = [3;5]; 创建5个元素为3的数组。
当数组索引超过len时，rust会直接中止程序而避免直接访问无效内存。

# lesson_7
```rust
    fn main(){
        let y = 6;
    }
// -- 
    fn main(){
        let y = {
            let x = 3;
            x + 1
        };
    }
```

第二段 x+1 没有冒号，表达式不包括分号，{}里是一个block，如果在后面加分号，就变成语句了，就不会返回值。
这里是表达式和语句的概念。

函数结束没有加分号，是作为结果返回的表达式，加了就变成语句了

# lesson_8

控制流， if不需要加() .
rust不会自动尝试将不是布尔类型的值转换为布尔类型.

loop、while、for

* 从loop循环中返回值
* while 非常常见，除了省略参数括号，无异。while在条件为真时重复。
* for访问数组比起while起码不会出现数组越界等问题。 
# lesson_9

开篇预告了几个新的概念：借用、切片、和RUst在内存中布局数据的方式。

## 什么是所有权

看了一些介绍，大概Rust是有一个编译期间有检查工作，不会对运行时产生开销，这一套东西，是特定规则的所有权系统管理。

## 堆 ｜ 栈

栈和堆，是运行时的可以使用的内存空间结构。我们常常是否容易把这段内存空间结构和数据结构中的栈堆概念混淆。其实可以进一步抽象出来，不用管其细节。我们在对数据操作，要想保存或者执行，将一些东西放入栈中，这个栈假设一个抽屉，放东西的，把东西放进去。然后，这时候的数据结构的栈可以出来解释，这个抽屉有特定的规则，是隔壁阿强规定的，“后进后出”。那么再回到编码时的栈堆概念，我们频繁的去操作数据，总会有些数据不跟上整齐的队伍里。比如我们想： 1，2，3。进去的 3，2，1 出来的。
如果突然来了个需求，1，2，3 进去的，要我们 1，3，2 出来，那怎么办？ 那是不是按照栈的顺序，先把3拿出来，把2拿出来，才能拿到 1 啊。所以输出1，3，2的需求，就麻烦多了，不仅人觉得是麻烦，计算机也会觉得麻烦，毕竟计算多了，电费也多了。

然后栈中的数据都必须有一个已知且固定的大小。一些没有确定大小的数据，在编译期就会识别出来，存在了堆里。

> 所谓明修栈道，所以栈中数据都必须明明白白，而不明不白的，就堆在一起。 ---微信读书Hei-542评论。

那么不明不白的堆是通过系统分配的，分配完给个地址能让我们找到要的数据，这个地址是放在了栈中，这样我们原本的 1，2，3 样东西都是指针的话，就能表示 3个不明不白 的垃圾堆的地址。 
然后我们考虑下性能，往栈上推入数据比堆上进行分配要更有效率。
这里面有个细节，同样是读取，或者保存一个数据，我们需要一个地址（位置），和合适的空间才能放入东西，我们考虑一下，放入栈中和堆中？ 想之前，有个提示，栈有个优势，栈的顶端有一个位置，是始终能返回一个地址的地方，只要把一个东西放进去，这个东西就自动伴随着这个地址。但是堆的话就麻烦多了，要系统帮它找一个大小合适的空间，东西多的时候又要调度，调整，一时半会没法整出来，好了，当放进去又要去栈那里弄个地址，给它贴上标签，实属麻烦了。

（由于缓存的缘故，指令在内存中跳转的次数越多，性能就越差。许多系统编程语言都需要开发者尽可能地记录分配的堆空间，最小化堆上的冗余数据。而掌握rust后，栈堆的考虑和负担将交给所有权和一些工具来减轻。）

## 所有权 (lesson_10)

* RUst中的每个值都有一个对应的变量作为它的所有者。
* 在同一时间内，值有且仅有一个所有者。
* 当所有者离开自己的作用域时，它持有的值就会被释放掉。

当我们简单地将s绑定字面量"Hello"，是不可变的，假设这里后续要更改，而String是可变的，是因为它们采用了不同的内存处理方式。我们在编写代码时，并不会都预先写了Hello这个字面量供使用，常常需要通过用户的输入来处理数据。
对于字符串字面量而言，编写代码已经写进去的，在编译时是已经知道了内容，所以这部分可以硬编码的文本直接嵌入到可执行文件中，所以访问字符串字面量是异常高效的。
当调用from函数，函数回请求自己需要的内存空间。也就是程序员来发起对内存的分配请求。这一个操作包括两段比较明显的数据处理：分配和归还。
* 使用的内存是由操作系统运行时动态分配出来的。
* 用完String时，要将这些内存归还回操作系统。
那么每当在变量离开它的作用域后就会进行释放，执行drop()函数。 （Rust会在作用域结束的地方调用）

## 二次释放

到了变量s1，s2离开自己的作用域时，到了函数结束的地方，也就是这行，就会执行drop释放函数，但是这里s1，s2都指向了同一个堆数据，就会尝试去重复释放，这就是臭名昭著的二次释放。
* 所以，rust为了避免重复浪费效能，又要安全，就想了新方法。当s2创建完毕，s1就会被简单地废弃。所以在以下代码会报错

```rust
    let s1 = String::from("123");
    let s2 = s1;
    println!("{}",s1); // err
```

那么这样子如何复制指针作一次浅度拷贝呢？rust的新概念“move”，s1移动到s2。

* Rust永远不会自动地创建数据的深度拷贝，每一次赋值都是高效可见的。

为什么在
```rust 

    let x = 5;
    let y = x;

```
这里5也能绑定到y呢？对于整型的类型，这里编译时就知道大小，完整地存储在栈中，这些值的复制操作无异于深浅拷贝、调用clone函数。
有哪些是无异于的： 
+ 所有的整数类型
+ bool
+ char
+ 所有浮点数
+ 如果元组包含的字段类型都是可以Copy的，那么这个元组也是Copy。

# lesson_12
所有权和函数，每次赋值都会转移所有权，如果每次都将持有堆数据的所有权在函数中传来传去有点繁琐，要将结果返回。也可以使用元组同时返回多个值。

# lesson_13_1

## 引用和借用

& 引用不持有所有权，在rust中，这种通过引用传递参数给函数的方式叫借用。

# lesson_13_2

## 可变引用

+ 对于作用域中的特定数据，一次声明一个可变引用
```rust
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;   
    // 错误
```

可以通过{}创建新的作用域范围。
* 不能在拥有不可变引用的同时创建可变引用

```rust
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;   
    let r3 = &mut s;
    // 错误
```

# lesson_14_1

悬垂引用状态： 指针指向曾经存在的内存地址，但是这处内存已经被释放或者已经被重新分配利用了。

# lesson_14_2

## 切片

# 由于需要加速个人的学习进度，就先不进行详细的学习分享，后续再详细补充。

# [所有权、生命周期, 温故而知新](https://rustcc.cn/article?id=2eda3337-3f8a-4be2-889e-6a232ef843e5)

## 方法 

传统class将方法都写在class里，写在一起，而Rust使用分离的方式，可以给予使用者极高的灵活度。

## 泛型

与往常的语言一样，rust也有泛型语法。
在T类型上需要比TS更多的说明才能操作使用，就是语义更明确一点，比如a+b，a: T , b: T，需要对这个T说明好加法的操作

## trait

特征: 共享行为
孤儿原则: 作用域上的限制应用，类型要实现特征，要看定义的作用域，如果定义所在的作用域不同，那么就应用不了。
作为函数参数
然后介绍特征的约束
&impl Summary 是语法糖，原本它是对trait的泛型书写。

### 元组

括号，取值 numbers.1

## update syntax 

对象拓展签名的

```rust
 let order_template = create_order_template(); 
 // TODO: Create your own order using the update syntax and template above!
 let your_order = Order {
     name: String::from("Hacker in Rust"),
     count: 1,
     ..order_template
     };
```
=======
* The "mod" keyword declares a submodule
-> mod关键字将其声明一个子模块
如果 mod my_module;
那么编译器会去 my_module.rs 或者 my_module/mod.rs 里面找

## 第二课
用mod.rs
>>>>>>> 第二课
