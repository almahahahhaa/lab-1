use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// This is required so that `CreationError` can implement `Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// Correct return type for `main`
fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    
    // Properly handle potential parsing errors
    let x: i64 = pretend_user_input.parse()?;
    
    // Properly handle potential creation errors
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    
    Ok(())
}
