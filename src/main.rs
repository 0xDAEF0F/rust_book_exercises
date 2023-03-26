// 5.2 An Example Using Structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "Method1: The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect2 = (30, 50);
    println!(
        "Method2: The area of the rectangle is {} square pixels.",
        area2(rect2)
    );

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "Method3: The area of the rectangle is {} square pixels.",
        area3(&rect3)
    );
    println!("the rectangle is {:?}", rect3);
    // debugging
    dbg!(&rect3);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
