// Link to `liblibrary`, import items under the `library` module
extern crate library;

fn main() {
    library::public_function();

    // Error! `private_function` is private
    //library::private_function();

    library::indirect_access();
}

// If you try to compile this on its own, it'll fail. 
