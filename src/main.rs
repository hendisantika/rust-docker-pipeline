fn main() {
    println!("Hello, world!");
    println!("Change this string to trigger recompilation instead of using Docker cache layers");
    println!("{:}", chrono::Utc::now())
}
