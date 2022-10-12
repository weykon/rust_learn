fn create_safe_name(name: String) -> (String, String) {
    let mut new_one = String::new();
    for c in name.chars() {
        match c {
            'a' | 'z' => {}
            _ => {
                new_one.push(c);
            }
        }
    }
    (new_one, name)
}

fn just_apply_name(name: String) -> String {
    print!("{}", name);

    name
}

fn main() {
    let (adjective, name) = create_safe_name("Nameee".to_owned());
    // if above did not export the name, it will only return the adjective name
    // and below here will use multiple times, but
    // we can fine the name, just be referenced, only read, 
    // so we can export the name
    // because there is no one borrowed it.

    // So how is the borrow behavior.
    // There is a motion direct to see, 
    // which the call is from which target.
    // like this:  name.chars() , that export something by itself,
    // and then println, like a motion catch it and do something
    // maybe that is the point. 
    let process_name = just_apply_name(name.clone());    

    println!("{}", adjective);
    println!("{}", name); // if follow dynamic type script language, it will error;
    println!("{}", process_name);
}
