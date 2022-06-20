macro_rules! MyMacroRule {
    ($name1: expr , $name2:expr) => {
        if !$name1 {
            $name2;
        }
    };
}

fn main() {
    let (a, b) = (1, 2);
    MyMacroRule!(a > b, { b - a })
}
