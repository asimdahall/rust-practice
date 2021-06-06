#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}
fn main() {
    let rect = Rect {
        width: 30,
        height: 30,
    };
    println!("The rect is {:#?}", rect);
    println!("The area of the rectangle is {} sq unit", area(&rect))
}

fn area(Rect { width, height }: &Rect) -> u32 {
    width * height
}
