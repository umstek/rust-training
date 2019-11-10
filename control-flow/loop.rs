fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        // This throws a warning - unreachable statement. 
        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // Break is for loops, just like return is for functions
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    // While loop - this is the famous fizz-buzz question. HackerRank has it. 
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }

    // For loop - haskell-like ranges and pythonic `in` syntax.
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let c1 = 1;
    let c = 100;

    // End-inclusive range... but why? this is ugly. We can avoid the `+ 1` yes. 
    // Good thing is, ranges work with variables, not just numbers. 
    // For loop cannot get arbitrary conditions :/ but that's okay, while `while` is around. 
    for n in c1..=c {
        if n % 15 == 0 {
            println!("=fizzbuzz");
        } else if n % 3 == 0 {
            println!("=fizz");
        } else if n % 5 == 0 {
            println!("=buzz");
        } else {
            println!("={}", n);
        }
    }
    
    // I get it. One `for` loop to handle both for and foreach. Again pythonic. 
    let names = vec!["Bob", "Frank", "Ferris"];
    
    // This borrows each element of the collection through each iteration. 
    // Thus leaving the collection untouched and available for reuse after the loop. 
    // Oh. 
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
                // Trailing commas :(
        }
    }

    // The move semantics!
    // This consumes the collection so that on each iteration the exact data is provided. 
    // Once the collection has been consumed it is no longer available for reuse as 
    // it has been 'moved' within the loop.
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us! //"),
            _ => println!("Hello {}", name),
        }
    }

    let mut names = vec!["Bob", "Frank", "Ferris"]; // Shadows
    // This mutably borrows each element of the collection, allowing for the collection to be modified in place.
    // Okay, wth I don't understand anymore. 
    for name in names.iter_mut() {
        // Assign to indirected pointer...?
        // Note that the vector is mutable. 
        *name = match name {
            // What is `&mut "Ferris"` doing here, and why was it `&"Ferris"` in .iter()? 
            // Is tis how pattern matching is done for a mutable variable?
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
