fn main() {}

mod tests {
    #[test]
    fn one() {
        let a = vec![1, 2, 3];

        let b = vec![3, 2, 1];

        for (a, b) in a.iter().zip(b.iter()) {
            println!("{},{}", a, b);
        }

        // 1,3
        // 2,2
        // 3,1
    }
    #[test]
    fn watch_TWO() {
        let a = vec![1, 2, 3, 4, 6];
        let b = vec![3, 2, 1];

        //zip: 他会忽略多出来的
        for o in a.iter().zip(b.iter()) {
            println!("{:?}", o);
        }

        let mut a_iter = a.iter();
        let mut b_iter = b.iter();
        while let Some(x) = a_iter.next() {
            println!("{}", x);
            if let Some(y) = b_iter.next() {
                println!("{}", y);
            } else {
                break;
            }
        }
    }

    #[test]
    fn what_drain (){
        let mut a = "abc".to_owned();
        a.drain(..1);
        println!("{}",a);
    }
}
