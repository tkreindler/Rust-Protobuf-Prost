use std::error;

// Allow any error
fn main() -> Result<(), Box<dyn error::Error>> {
    // Compile proto buf and return Err(TODO) when there's an error
    // TODO: figure out error type and why result only has one parameter
    prost_build::compile_protos(&["src/address_book.proto"], &["src/"])?;

    // All good so return ok
    Ok(())
}
