fn main() {
    // The "\t" character enters a tab.
    println!("Celsius\t| Fahrenheit"); 
    // Note: as of Rust 1.28.0 (2018-08-02), Iterator::step_by()
    // has been stabilized, which means we can get specify that the
    // iterator should return numners in steps of 10.
    // E.g., `for celsius in (0..101).step_by(10)`
    for celsius in 0..101 {
        if celsius % 10 == 0 {
            let fahrenheit = (celsius as f32) * 9.0/5.0 + 32.0;
            println!("{} C\t| {} F", celsius, fahrenheit);
        }
    }

    // If you are using Rust 1.28.0 you can use this alternate code
    //
    // for celsius in (0..101).step_by(10) {
    //     let fahrenheit = (celsius as f32) * 9.0/5.0 + 32.0;
    //     println!("{} C\t| {} F", celsius, fahrenheit);
    // }
}
