use std::env;

fn main() {
    // Collect arguments into a Vector (argv equivalent)
    let args: Vec<String> = env::args().collect();

    // Get total argument count (argc equivalent)
    let argc = args.len();

    println!("Argument count (argc): {}", argc);

    // Program name / path is always at index 0 (argv[0])
    if argc > 0 {
        println!("Executable path (argv[0]): {}", args[0]);
    }

    println!("\nAll arguments:");
    for (i, arg) in args.iter().enumerate() {
        println!("  argv[{}]: {}", i, arg);
    }
}
