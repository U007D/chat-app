type Result<T, E = Error> = std::result::Result<T, E>;
enum Error {}
impl std::error::Error for Error{} // implement trait for type, Error type is now impls std error trait

fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}
