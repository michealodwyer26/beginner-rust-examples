use std::io;

fn main() {
    let mut x1 = String::new();
    let mut y1 = String::new();
    let mut points_count = 0;

    println!("After entering at least 3 points, enter a blank to display the perimeter.");

    println!("Enter the x value.");
    io::stdin().read_line(&mut x1)
        .expect("Failed to read line.");
    
    // We create a mutable binding for x1, and y1, because later on
    // these values will be reset.
    let mut x1: i32 = match x1.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid x value entered.")
    };

    println!("Enter the y value.");
    io::stdin().read_line(&mut y1)
        .expect("Failed to read line.");
    
    let mut y1: i32 = match y1.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid y value entered.")
    };
    
    points_count += 1;

    // We initialize the perimeter to 0.0, because Rust won't let us take any 
    // chances, as regards uninitialized variables.
    let mut perimeter = 0.0;

    loop {
        // These values represent the next point.
        let mut x2 = String::new();
        let mut y2 = String::new();

        println!("Enter the x value.");
        io::stdin().read_line(&mut x2)
            .expect("Failed to read line.");
        
        // The user can only quit successfully by entering a blank line at this prompt.
        if x2.trim() == "" {
            // A polygon must have at least 3 points.
            if points_count >= 3{
                break;
            } else {
                panic!("At least 3 points are needed.");
            };
        }

        println!("Enter the y value.");
        io::stdin().read_line(&mut y2)
            .expect("Failed to read line.");

        let x2: i32 = match x2.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Invalid x value entered.")
        };
        let y2: i32 = match y2.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Invalid y value entered.")
        };
        
        // Increment perimeter using a simple distance formula
        // Note: onlt f32s or f64s have a sqrt() method which is why
        // we convert to that type.
        perimeter += (((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f32).sqrt();
        // Check if the point is the same as the last point entered
        if x1 == x2 && y1 == y2 {
            panic!("Duplicate point entered.");
        } else {
            // Reset x1 and y1 with x2 and y2 respectively.
            x1 = x2;
            y1 = y2;
        }

        points_count += 1;
    }

    println!("The perimeter is {}.", perimeter);
}
