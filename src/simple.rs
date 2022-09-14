// https://zhuanlan.zhihu.com/p/145592746 good

pub fn main() {
    let mut array: [i32; 3] = [1, 2, 3];

    let ref1: &[i32; 3] = &array;

    let ptr1: *const [i32; 3] = ref1 as *const [i32; 3];

    let ref2: &mut [i32; 3] = &mut array;

    let ptr2: *mut [i32; 3] = ref2 as *mut [i32; 3];


    // 再用 `as` 操作符将引用转换为裸指针：
    println!("here is the pointer: {:?}",ptr1);
}