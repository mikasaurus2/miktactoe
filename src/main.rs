use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    miktactoe::run_app()?;
    Ok(())
}
