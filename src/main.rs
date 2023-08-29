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
    fn what_drain() {
        let mut a = "abc".to_owned();
        a.drain(..1);
        println!("{}", a);
    }

    #[test]
    fn complex() {
        let upper = "Hello World".to_owned();
        let lowwer = "weykon".to_owned();
        let (b, mut s) = upper
            .chars()
            // 翻转 dlroW olleH
            .rev()
            // zip可以用，不过要自己去补齐差集。以下是用0的iter来补齐
            .zip(
                lowwer.chars().rev()
                // chain： 我一开始不解为何还需要chain这样来链接，其实就像是concat一样，命令式地去实现链接
                .chain(std::iter::repeat('0'))
            )
            // 所以现在的： dlroW olleH - nokyew00000.
            // 那么在zip的作用下，他是这样运行的 (d,n) -> (l,o) -> (r,y) -> ... -> (H,0)
            // 这样在fold的时候就是处理每一个元组了
            .fold((false, String::new()), |mut acc: (bool, String), (u, l)| {
                println!("acc: {:?}, u:{:?}, l:{:?}, ", acc,u,l);
                match (false, u, l) {
                    (false, '1', '1') => {
                        acc.0 = true;
                        acc.1.push('0');
                        (acc.0, acc.1)
                    }
                    (false, '1', '0') => {
                        acc.0 = false;
                        acc.1.push('1');
                        (acc.0, acc.1)
                    }
                    (false, '0', '1') => {
                        acc.0 = false;
                        acc.1.push('1');
                        (acc.0, acc.1)
                    }
                    (false, '0', '0') => {
                        acc.0 = false;
                        acc.1.push('0');
                        (acc.0, acc.1)
                    }
                    (true, '1', '1') => {
                        acc.0 = true;
                        acc.1.push('1');
                        (acc.0, acc.1)
                    }
                    (true, '1', '0') => {
                        acc.0 = true;
                        acc.1.push('0');
                        (acc.0, acc.1)
                    }
                    (true, '0', '1') => {
                        acc.0 = true;
                        acc.1.push('0');
                        (acc.0, acc.1)
                    }
                    (true, '0', '0') => {
                        acc.0 = false;
                        acc.1.push('1');
                        (acc.0, acc.1)
                    }
                    _ => panic!(),
                }
            });
            println!("b: {}, s:{}", b, s);
            assert_eq!(true,false);
    }
}
