fn main() {
    let y = {
        let x = 3;
        // no semicolon after 1, if you add a semicolon, it will become statement
        x+1
    };

    println!("The value of y is: {y}");
}
