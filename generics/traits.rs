// For my test
use std::fmt::Debug;

// Non-copyable types.
struct Empty;
struct Null;

// A trait generic over `T`.
trait DoubleDrop<T> {
    // Define a method on the caller type which takes an
    // additional single parameter `T` and does nothing with it.
    fn double_drop(self, _: T);
}

// Implement `DoubleDrop<T>` for any generic parameter `T` and
// caller `U`.
impl<T, U> DoubleDrop<T> for U {
    // This method takes ownership of both passed arguments,
    // deallocating both.
    fn double_drop(self, _: T) {}
}

// Okay, wait! Did you just implement a function for all types?

trait P {
    fn print1(&self);
}

impl<T: Debug> P for T {
    fn print1(&self) {
        println!("{:?}", self);
    }
}

// Let's see.

fn main() {
    let empty = Empty;
    let null  = Null;

    // Deallocate `empty` and `null`.
    empty.double_drop(null);

    //empty;
    //null;
    // ^ TODO: Try uncommenting these lines.

    // My test
    1.print1();
    // It works! 
    // We effectively gave a default `print1` implementation
    // for all those who extends (subtraits?) `Debug` trait.
    // Anyway this is useless. These types already had an actual implementation of
    // Debug trait. 
}
