fn main() {
    println!("Hello, world!!!");

// region:    --- returning values from loops
    // https://rust-book.cs.brown.edu/ch03-05-control-flow.html#returning-values-from-loops
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        println!("  ->> counter: {counter}");
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
// endregion: --- returning values from loops


}
