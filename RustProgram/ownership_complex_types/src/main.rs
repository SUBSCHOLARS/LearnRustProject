fn main() {
    let s = String::from("hello");

    // this kind of string can be mutated
    let mut s2=String::from("hello");
    s2.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s2}"); // this will print 'hello, world'
}
