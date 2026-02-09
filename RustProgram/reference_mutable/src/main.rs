fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problem, otheerwise the multiple mutable references at a time are not allowed
    let r2 = &mut s; 
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
