fn main() {
    let _tup: (i32, f64, u8)=(500,6.4,1);
    let (_x,y,_z)=_tup;

    println!("The value of y is: {y}");

    let _x: (i32, f64, u8)=(500,6.4,1);
    let _five_hundred=_x.0;
    let _six_point_four=_x.1;
    let _one=_x.2;
}
