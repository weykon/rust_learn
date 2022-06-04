pub trait Summary {
    fn summarize(&self) -> String;
    fn defalut_impl(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{},作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
    fn defalut_impl(&self) -> String {
        format!("{}这是默认实现的重载方法咯,", self.username);
    }
}

fn main() {
    let post = Post {
        title: "Rust语言简洁".to_string(),
        author: "SUnface".to_string(),
        content: "Rust棒极了！".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "好像微博没有Tweet好用".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
}
