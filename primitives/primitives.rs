fn main() {
    // let defines an immutable variable. Just like `const` in JS or TS. 
    // Type comes after the colon, looks TypeScriptish. 
    // Type inference <3. 

    // Variables can be type annotated.
    let logical: bool = true;

    // Short, but meaningful type names. 
    let a_float: f64 = 1.0;  // Regular annotation
    // Meaningful literal syntax. 
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;
    
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    // Error! The type of a variable can't be changed.
    // mutable = true;
    
    // Variables can be overwritten with shadowing.
    let mutable = true;

     // Integer addition
     println!("1 + 2 = {}", 1u32 + 2);

     // Integer subtraction
     println!("1 - 2 = {}", 1i32 - 2);
     // ^ Try changing `1i32` to `1u32` to see why the type is important
     // println!("1 - 2 = {}", 1u32 - 2);
     // Won't compile! Compile-time integer overflow checks!
 
     // Short-circuiting boolean logic, nothing fancy here
     println!("true AND false is {}", true && false);
     println!("true OR false is {}", true || false);
     println!("NOT true is {}", !true);
 
     // Bitwise operations, similar to most of other languages including C
     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
     println!("1 << 5 is {}", 1u32 << 5);
     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
     // P. S.: These binary numbers are in my dream syntax! 0x, 0o or 0b and suffix type
 
     // Use underscores to improve readability!
     println!("One million is written as {}", 1_000_000u32);

    // A tuple with a bunch of different types, just like in other languages, is a table row. 
    // This feels pythonic, parantheses take careof the type. 
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    // Values can be extracted from the tuple using tuple indexing. This syntax is different. 
    // I think the choice is more logical than using the array braces, because we don't need to allow ranges.
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Struct, but tuple? What is this?
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    // Arrays
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0, nothing crazy
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the size of the array
    println!("array size: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);
    // Slices with easy range syntax. 

    // Out of bound indexing causes compile error
    // println!("{}", xs[5]);
    // Compile-time array bounds check! O.o
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
// #[derive(Debug)] is like decorators in other languages. 

use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}