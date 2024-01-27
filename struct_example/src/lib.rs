use ic_cdk_macros::{query, export_candid};



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
fn get_area(width: u32,height: u32) -> String {
    let rectangle=Rectangle{
        width:width,
        height:height
    };

    format!("The area of the rectangle is {} square pixels.",rectangle.area())
}

#[query]
fn can_hold(width: u32,height: u32) -> String {
    let rectangle1=Rectangle{
        width:width,
        height:height
    };

    let rectangle2 =  Rectangle::square(30);

    format!("Can rectangle hold \"{}\" ",rectangle1.can_hold(&rectangle2))
}

// Generate did files

export_candid!();
