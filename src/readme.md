# feature 作为条件编译和可选依赖
Feature 可以通过 Cargo.toml 中的 [features] 部分来定义：其中每个 feature 通过列表的方式指定了它所能启用的其他 feature 或可选依赖。

假设我们有一个 2D 图像处理库，然后该库所支持的图片格式可以通过以下方式启用：
```
[features]
## 定义一个 feature : webp, 但它并没有启用其它 feature
webp = []
```

当定义了 webp 后，我们就可以在代码中通过 href="https://doc.rust-lang.org/stable/reference/conditional-compilation.html">cfg 表达式来进行条件编译。例如项目中的 lib.rs 可以使用以下代码对 webp 模块进行条件引入:
```
#[cfg(feature = "webp")]
pub mod webp;
```
#[cfg(feature = "webp")] 的含义是：只有在 webp feature 被定义后，以下的 webp 模块才能被引入进来。由于我们之前在 [features] 里定义了 webp，因此以上代码的 webp 模块会被成功引入。

在 Cargo.toml 中定义的 feature 会被 Cargo 通过命令行参数 --cfg 传给 rustc，最终由后者完成编译：rustc --cfg ...。若项目中的代码想要测试 feature 是否存在，可以使用 href="https://doc.rust-lang.org/stable/reference/conditional-compilation.html#the-cfg-attribute">cfg 属性或 href="https://doc.rust-lang.org/stable/std/macro.cfg.html">cfg 宏。

之前我们提到了一个 feature 还可以开启其他 feature，举个例子，例如 ICO 图片格式包含 BMP 和 PNG，因此当 ICO 图片格式被启用后，它还得确保启用 BMP 和 PNG 格式：
```
[features]
bmp = []
png = []
ico = ["bmp", "png"]
webp = []
```
对此，我们可以理解为： bmp 和 png 是开启 ico 的先决条件。