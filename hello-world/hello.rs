// Comments, same as most languages.
/*
 * Even
 * if 
 * multiline
 */

// fn (below) is better for a function definition. Why waste type for `function` like we do in javascript?
// But why not just the return type like everyone else does (C/C++, C#, Java, ...)? 
// Parameters are omitted in the function def, and also the return type!
// Neat and clean :)

/// Documentation comment - documents what comes below
/// Looks pretty much like C# doc comments to me...
fn main() {
    //! But, what does it mean by "to Generate library docs for the enclosing item"?
    
    // Macros, huh? I don't like magic. 
    // But at least it's short, unlike Java's System.out.println. 
    println!("Hello World!");
    println!("I'm a Rustacean!");

    // Print to standard error
    eprint!("Error? \nWhat?\n");
    // Esccapes work as usual. Without `ln` suffix, it doesn't append a newline. 

    // format! macro for strings, and println! to standard out. 
    // New format string. 
    println!("{number:>0width$}", number=1, width=6);
    // All std types derive something called `debug`. That is printed when `?` is used. 
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
    "Slater",
    "Christian",
    actor="actor's");
    // Have to check out this formatting syntax later; it's little strange, should compare with C# and Python 3. 
}
