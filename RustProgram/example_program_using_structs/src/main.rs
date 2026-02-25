#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1=30;
    let height1=50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );

    // refactoring with tuples
    let rect1=(30, 50);

    println!(
        "The are of the rectangle is {} quare pixels",
        area_tuples(rect1)
    );

    // refactoring with structs
    let rect2=Rectangle{
        width: 30,
        height: 50,
    };

    println!("rect2 is {rect2:?}");

    println!(
        "The area of the rectangle is {} square pixels",
        area_structs(&rect2)
    );

    let scale=2;
    let rect3=Rectangle{
        width: dbg!(30*scale),
        height: 50,
    };
    dbg!(&rect3);
}

fn area(width: u32, height: u32) -> u32 {
    width*height
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_structs(rectangle: &Rectangle) -> u32 {
    rectangle.width*rectangle.height
}


