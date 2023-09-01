mod course;
pub fn main() {
    let mut name = format!( "add");

    update(&mut name);
    println!("{}", name);

    let r = &mut name;

    update(r);
    update(r);

    println!("{}", name);

    course::main();
}

fn update(name: &mut String) {
    name.push_str(".");
}
 


// here is the comprehend
// Ownership: control all access, will free when done 
// Shared reference: many readers, no writers
// Mutable reference: no other readers, one writer (???)

// probably should clear about: 
// 1. Create a shared reference to X "read locks" X.
// 2. Create a mutable reference to X "writes locks" X.
// two point both locking lasts until reference goes out of scope