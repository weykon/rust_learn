struct SomeType;

trait 形式中的行为<'周期, T> {
    fn 放屁(arg: T);

    fn 带周期放屁(arg: &'周期 T);

    fn 带周期放屁下的固定u32类型(arg: &'周期 u32);
}

impl<'星期一> 形式中的行为<'星期一,u8> for SomeType { 
    fn 放屁(arg: u8){}

    fn 带周期放屁(arg: &'星期一 u8){}

    fn 带周期放屁下的固定u32类型(arg: &'星期一 u32){}
}

fn main( ){
    
}
