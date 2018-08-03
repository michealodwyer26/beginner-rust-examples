use std::io;

fn main() {
    // Since we still have to get the numbers from the user,
    // we don't use the regular vec! macro, although we still can
    // we add in some annotations.
    let mut numbers = Vec::new();
    
    println!("Enter each integer by typing the number and pressing enter.");
    println!("When you are finished type in 0 (which won't be used in the calculation).");

    loop {
        let mut num = String::new();
        io::stdin().read_line(&mut num)
            .expect("Failed to read line.");

        let num: i32 = match num.trim().parse() {
            Ok(0) => break,
            Ok(num) => num,
            Err(_) => panic!("Invalid float entry.")
        };

        numbers.push(num);
    }

    if numbers.is_empty() {
        panic!("Only number entered was 0");
    }

    let mut sum = 0;
    for num in &numbers {
        sum += num;
    }
    let average = sum as f64 / numbers.len() as f64;
    println!("The average: {}", average);
}
