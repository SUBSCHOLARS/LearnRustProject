use std::io;

fn main() {
    loop{
        println!("Please input farenheit: ");
        // mut means mutable, which we can change the value
        let mut farenheit=String::new();

        io::stdin()
            .read_line(&mut farenheit)
            .expect("Failed to read line");

        let farenheit: i32=match farenheit.trim().parse() {
            Ok(num) => num,
            Err(_)=>continue,
        };
        let celcius=farenheit_to_celsius(farenheit);
        println!("{farenheit}F is {celcius}C");
        break;
    }
}

fn farenheit_to_celsius(f :i32) -> f32 {
    return ((f as f32)-32.0)*5.0/9.0;
}