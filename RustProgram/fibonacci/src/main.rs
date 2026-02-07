use std::io;
fn main() {
    loop{
        println!("Please input n: ");
        // mut means mutable, which we can change the value
        let mut n=String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n: u32=match n.trim().parse() {
            Ok(num) => num,
            Err(_)=>continue,
        };
        let n_fib=nth_fibonacci_value(n);
        print!("The nth value of fibonacch is: {n_fib}");
        break;
    }
}

fn nth_fibonacci_value(n :u32) -> u32 {
    return ((1.0_f32/5.0_f32.sqrt())*( (((1.0+5.0_f32.sqrt())/2.0).powf( n as f32) + ((1.0-5.0_f32.sqrt())/2.0).powf( n as f32) ))) as u32
}
