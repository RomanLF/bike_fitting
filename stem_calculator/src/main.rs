use std::env;
use std::process;

// Function to compute the new stem length
fn compute_new_stem_length(
    R: f64,       // Frame reach (horizontal distance from seat to head tube)
    s_a: f64,     // Original stem length
    h_a: f64,     // Original handlebar width
    h_b: f64      // New handlebar width
) -> f64 {
    // Calculate the required new stem length (s_b) to maintain the same effective reach
    let term_1 = (R + s_a).powi(2);           // (R + s_a)^2
    let term_2 = (h_a / 2.0).powi(2);         // (h_a / 2)^2
    let term_3 = (h_b / 2.0).powi(2);         // (h_b / 2)^2

    // Calculate the new stem length using the formula
    let s_b = (term_1 + term_2 - term_3).sqrt() - R;

    s_b
}

fn print_usage(program_name: &str) {
    println!("Usage: {} <R> <s_a> <h_a> <h_b>", program_name);
    println!("Where:");
    println!("  <R>   - Frame reach (mm)");
    println!("  <s_a> - Original stem length (mm)");
    println!("  <h_a> - Original handlebar width (mm)");
    println!("  <h_b> - New handlebar width (mm)");
}

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        let program_name = &args[0];
        print_usage(program_name);
        process::exit(1);
    }

    // Parse command-line arguments
    let R: f64 = args[1].parse().unwrap_or_else(|_| {
        println!("Error: Invalid input for frame reach (R). Must be a number.");
        process::exit(1);
    });

    let s_a: f64 = args[2].parse().unwrap_or_else(|_| {
        println!("Error: Invalid input for original stem length (s_a). Must be a number.");
        process::exit(1);
    });

    let h_a: f64 = args[3].parse().unwrap_or_else(|_| {
        println!("Error: Invalid input for original handlebar width (h_a). Must be a number.");
        process::exit(1);
    });

    let h_b: f64 = args[4].parse().unwrap_or_else(|_| {
        println!("Error: Invalid input for new handlebar width (h_b). Must be a number.");
        process::exit(1);
    });

    // Compute the new stem length
    let s_b = compute_new_stem_length(R, s_a, h_a, h_b);

    // Output the result
    println!("The new stem length should be: {:.2} mm", s_b);
}
