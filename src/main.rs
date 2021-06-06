#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, second_rect: &Rect) -> bool {
        if self.area() > second_rect.area() {
            return true;
        }
        false
    }
}
fn main() {
    let rect = Rect {
        width: 30,
        height: 30,
    };
    let second_rect = Rect {
        width: 20,
        height: 30,
    };
    println!("The rect is {:#?}", rect);
    println!("The area of the rectangle is {} sq unit", rect.area());
    if rect.can_hold(&second_rect) {
        println!("Yes the first rectangle can hold the second one")
    } else {
        println!("NO, SORRY :( the first rectangle cannot hold the second one");
    }
}
