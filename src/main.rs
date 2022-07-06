use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, EnumIter)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
	Coin::Penny => {
	    println!("{:>14}{:*^11}", "Lucky Penny:", format_args!("{:?}", coin));
	    1
	}
	Coin::Nickel => 5,
	Coin::Dime => 10,
	Coin::Quarter => 25,
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
	self.width * self.height
    }
}

fn enum_coins() {
    for k in Coin::iter() {
	println!("{: >7} value:{:>02} cts.", format!("{:?}", k), value_in_cents(&k));
    }
}

fn rect_area() {
    let r = Rectangle{width:10, height:20};
    println!("Rectangle({},{}):{:?}->area:{}", r.width, r.height, r, r.area());
}

fn plus_one(opt: Option<u32>) -> Option<u32> {
    match opt {
	None => None,
	Some(_) => opt.map(|o| o+1),
    }
}

fn match_option() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?} {:?} {:?}", five, six, none);
    println!("{:?} {:?} {:?}", five.unwrap(), six.unwrap(), none);
}

fn go() {
    rect_area();
    enum_coins();
    match_option();
}

fn main() {
    go();
}
