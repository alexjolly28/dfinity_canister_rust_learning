use ic_cdk_macros::{query, export_candid};
use ic_cdk::export::candid::{CandidType, Deserialize};


#[derive(CandidType, Deserialize)]
struct Rectangle{
    width : u32,
    height : u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

#[query]
fn get_area(rectangle:Rectangle) -> String {


    format!("The area of the rectangle is {} square pixels.",rectangle.area())
}

#[query]
fn can_hold(rectangle:Rectangle) -> String {

    let rectangle1 =  Rectangle::square(30);

    format!("Can rectangle hold \"{}\" ",rectangle.can_hold(&rectangle1))
}

// Generate did files

export_candid!();
