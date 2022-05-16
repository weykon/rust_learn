fn main() {
    
    let a = [10,20,30,40,50];

    for element in a.iter() {
        println!("The result is {}", element);
    }

    for element in (1..4).rev(){
        println!("{}!",element)
    }
}
