// struct ClothData {
//     data: Vec<u8>,
// }
// struct PropertyData {
//     data: Vec<i32>,
// }

type ClothData = {
    data: Uint8Array,
}
type PropertyData = {
    data: Uint32Array,
}
function ViewData<T extends { data: number }>(data: T) {
    console.log(data.data);
}
function printData(data: Uint8Array): void;
function printData(data: Int32Array): void;
function printData(data: Uint8Array | Int32Array): void {
    if (data instanceof Uint8Array) {
        console.log(data);
    } else if (data instanceof Int32Array) {
        console.log(data);
    }
}
// 在 Rust 中，trait 对象是通过使用 &dyn Trait 或 Box<dyn Trait> 来表示的。这样可以在运行时进行动态分发，根据实际类型调用相应的方法。
// 在 TypeScript 中，由于它是一种静态类型语言，没有直接的动态分发机制，因此无法完全模拟 Rust 中的动态分发。最接近的替代方法是使用函数重载和函数签名来处理不同类型的数据