use std::env;
use std::io;

fn main() -> io::Result<()> {
    // Get the current working directory
    let current_dir = env::current_dir()?;

    // Print the current working directory
    println!("Current working directory: {}", current_dir.display());

    Ok(())
}
