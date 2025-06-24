fn main() {
    if let Err(e) = uniq::get_args().and_then(uniq::run) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
