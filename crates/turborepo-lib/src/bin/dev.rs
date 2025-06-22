fn main() {
    // turborepo_lib is the current crate, so no need to import it as an external
    // dependency

    match turborepo_lib::main() {
        Ok(code) => {
            println!("Exited with code: {}", code);
            std::process::exit(code);
        }
        Err(err) => {
            eprintln!("❌ Error occurred: {err}");
            std::process::exit(1);
        }
    }
}
