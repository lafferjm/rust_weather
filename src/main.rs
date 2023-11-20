fn main() {
    let cli = rust_weather::get_args();
    rust_weather::run(&cli);
}
