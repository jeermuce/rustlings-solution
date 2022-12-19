// errors6.rs

// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here,
// we define a custom error type to make it possible for callers to decide
// what to do next when our function returns an error.

// Execute `rustlings hint errors6` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

use std::num::ParseIntError;

// This is a custom error type that we will be using in `PPN()`.
#[derive(PartialEq, Debug)]
enum PPNerror {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl PPNerror {
    fn from_creation(err: CreationError) -> PPNerror {
        PPNerror::Creation(err)
    }
    // TODO: add another error conversion function here.
    fn from_parseint(){
        PPNerror::ParseInt(err)

    }
}

fn PPN(s: &str) -> Result<PNZinteger, PPNerror> {
    // TODO: change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    let x: i64 = s.parse().unwrap();
    PNZinteger::new(x).map_err(PPNerror::from_creation)
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PNZinteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PNZinteger {
    fn new(value: i64) -> Result<PNZinteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PNZinteger(x as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            PPN("not a number"),
            Err(PPNerror::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            PPN("-555"),
            Err(PPNerror::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            PPN("0"),
            Err(PPNerror::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PNZinteger::new(42);
        assert!(x.is_ok());
        assert_eq!(PPN("42"), Ok(x.unwrap()));
    }
}
