// Traits are basically interfaces. So, functions can also implement interfaces!?
// I have seen this in typescript. `interface I { (x: type, ...): outType; };
// C# delegates, anyone? `delegate outType I(type x);`.

// Return type says, "this returns something that implements these traits (interfaces)". 
// NOT "this return an instance of this trait", as it would in other languages. 
// So traits are considered something higher level to the types? 
// Makes sense why we had to use the generic-specification (oxymoron?) or the where clause. 

// That means the below should work instead of `fn call_me<F: Fn()>(f: F) { f(); }`.
// Damn it DOES work. 
fn call_me(f: impl Fn()) {
    f();
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

// These don't work though. ?

// fn create_fn2<F>() -> F where F: Fn() {
//     let text = "Fn".to_owned();

//     move || println!("This is a: {}", text)
// }

// fn create_fn2<F: Fn()>() -> F {
//     let text = "Fn".to_owned();

//     move || println!("This is a: {}", text)
// }

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();
    // create_fn2();

    fn_plain();
    fn_mut();
    fn_once();

    // Added to support description at the top. 
    let closure = || println!("I'm a closure!");

    call_me(closure);
}
