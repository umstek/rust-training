use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

// i.e.: Implement generic interface From with i32 for Number class
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    // We get `into` for free when we define `from` for the type. 
    // Type declaration must be there for the compiler to understand the output type. 
    println!("My number is {:?}", num);
}
