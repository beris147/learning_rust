pub fn run() {
    // Print to console
    println!("Hello from the print.rs file!");

    // Formatting
    println!("Number: {}", 1);
    println!("My name is {} and I am from {}", "Edgar", "Mexico");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Edgar", "Mexico", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Edgar",
        activity = "Videogames"
    );

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (23, true, "Edgar"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
