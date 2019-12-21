// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.

// I wrote this as fn apply<F: Fn()>(f: F) { ... } omitting the where clause. 
// It worked. 
fn apply<F>(f: F) where
    F: Fn() {
    // F can be FnMut or FnOnce because they are less restrictive. 
    f();
}

fn main() {
    let x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || println!("{}", x);

    apply(print);
}
