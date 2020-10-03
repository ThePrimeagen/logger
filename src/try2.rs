#[derive(Debug)]
enum ParsedLineError {
    BadId,
    BadLine,
    BadClassName,
    BadState,
    BadNumber,
}

impl fmt::Display for ParsedLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsedLineError::BadLine => write!(f, "Bad Line"),
            ParsedLineError::BadId => write!(f, "Oh no! your id's are big time suck."),
            ParsedLineError::BadClassName => write!(f, "Your classname was weak"),
            ParsedLineError::BadState => write!(f, "F U Mccannch (BadState)"),
            ParsedLineError::BadNumber => write!(f, "F U Mccannch (BadState)"),
        }
    }
}

impl Error for ParsedLineError {}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let x = 5;

    for line in stdin.lock().lines() {
        let line = line?;
        let parsed_line = parse(&line)?;

        print!("ParsedLine {:?}", parsed_line);
    }

    return Ok(());
}
