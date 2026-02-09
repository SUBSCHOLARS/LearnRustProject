fn main() {
    let my_string=String::from("hello world");

    // first_word works on slices of Strings, whether partial or whole
    let word=first_word(&my_string[0..6]);
    let word=first_word(&my_string[..]);

    // first_word also works on references to Strings, which are equivalent to whole slices of Strings
    let word=first_word(&my_string);

    let my_string_literal="hello world";

    // first_word works on slices of string literals, whether partial or whole
    let word=first_word(&my_string_literal[0..6]);
    let word=first_word(&my_string_literal[..]);

    // BEcause string literals are slices alread, this works too
    let word=first_word(my_string_literal);
}
fn first_word(s: &str) -> &str {
    let bytes=s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}