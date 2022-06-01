#[cfg(target_os = "linux")]
fn are_you_on_linux(){
    println!("You are!");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux(){
    println!("You Not!");
}

fn main(){

}