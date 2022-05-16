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

+ 从loop循环中返回值
+ while 非常常见，除了省略参数括号，无异。while在条件为真时重复。
+ for访问数组比起while起码不会出现数组越界等问题。 