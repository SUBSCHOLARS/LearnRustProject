fn main() {
    let reference_to_nothing=dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s=String::from("hello");  // s is a new String

    return &s;  // we return a reference to the String, s
} // Here, s goees out of scope and is dropped, so its memory goes away. Danger!
// The solution here is to return String directly