/// 这个文件我也看不清说了什么了，
/// 现在试试看看，
/// RefCell这个是在看来比较少用的一个，它是在一个不可变的引用中，试图去改变他的值。
/// 
/// 比如下来的Messenger的trait，他的定义是一个send方法，
/// 接受的是一个&str，而且这里是没有写 &mut str。
/// 
/// 似乎是，如果一个数据他已经是源代码写好是不可变的引用，
/// 此时又需要去改变里面的值的话，就用这个RefCell来包裹一下，
/// 这样在去取borrow_mut的时候是拿一个RefMut，如果没有问题的话
/// 也就是说没有其他活动的引用
/// 就能拿到可变的引用，不然就会panic。
/// https://doc.rust-lang.org/stable/core/cell/index.html 
/// 这里解释有
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

fn main(){
    
}