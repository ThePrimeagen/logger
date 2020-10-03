use std::{error::Error, str::FromStr};
use std::fmt::{self, Debug};
use std::io::{self, BufRead};
use std::convert::From;
use std::num::ParseIntError;

#[derive(Debug)]
enum ParsedLineError {
    _BadId,
    _BadLine,
    _BadClassName,
    _BadState,
    BadNumber,
    BadSeparator,
    NotEnoughCharacters,
}

#[derive(Debug)]
struct ParsedLine<'a> {
    id: i32,
    parent_id: Option<i32>,  // -1 = not there
    parent: Option<&'a str>, // -1 = not there
    class_name: &'a str,
    function_name: &'a str,
    state: Vec<&'a str>,
    args: Vec<&'a str>,
}

impl fmt::Display for ParsedLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsedLineError::_BadLine => write!(f, "Bad Line"),
            ParsedLineError::_BadId => write!(f, "Oh no! your id's are big time suck."),
            ParsedLineError::_BadClassName => write!(f, "Your classname was weak"),
            ParsedLineError::_BadState => write!(f, "F U Mccannch (BadState)"),
            ParsedLineError::BadNumber => write!(f, "F U Mccannch (BadState)"),
            ParsedLineError::BadSeparator => write!(f, "Unable to find separator"),
            ParsedLineError::NotEnoughCharacters => write!(f, "Not enough characters in string."),
        }
    }
}

impl From<ParseIntError> for ParsedLineError {
     fn from(_: ParseIntError) -> Self {
        return ParsedLineError::BadNumber;
    }
}

impl Error for ParsedLineError {}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line?;
        match match parse(&line) {
            Ok(v) => Some(v),
            Err(e) => {
                print!("I have found an erro, and my stomach hurts! {:?}\n", e);
                None
            }
        } {
            Some(v) => print!("ParsedLine {:?}", v),
            _ => {}
        }
    }

    return Ok(());
}

fn parse(line: &str) -> Result<ParsedLine, Box<dyn Error>> {

    // <ID> <Class> <function name> <state args> <arguments to function>
    // state args
    // <timestamp> <count>:<length>:<object><length>:<object>... count <whitespace separator>
    let (
        _,
        line
    ) = parse_number::<u64>(line)?;
    let line = pop_separator(line, ' ')?;

    let (
        id,
        line
    ) = parse_number::<i32>(line)?;
    let line = pop_separator(line, ' ')?;
    let (
        class,
        line
    ) = parse_class_name(line)?;
    let line = pop_separator(line, ' ')?;

    let (
        function_name,
        line,
    ) = take_until_whitespace(line)?;
    let line = pop_separator(line, ' ')?;

    let (
        states,
        line
    ) = parse_state(line)?;
    let line = pop_separator(line, ' ')?;

    let (
        args,
        line,
    ) = parse_state(line)?;

    assert!(
        line.len() == 0,
        format!("I expected line to be 0 but got {}: with contents {}", line.len(), line));

    return Ok(ParsedLine {
        function_name,
        class_name: class.class_name,
        id,
        parent: class.parent_class_name,
        parent_id: class.parent_id,
        state: states,
        args,
    });
}

// Big question on making this better
#[derive(Debug)]
struct ParsedClassName<'a> {
    class_name: &'a str,
    parent_id: Option<i32>,
    parent_class_name: Option<&'a str>,
}

fn parse_state<'a>(line: &'a str) -> Result<(Vec<&'a str>, &str), ParsedLineError> {
    let (
        state_var_count,
        remaining
    ) = parse_number::<i32>(line)?;

    let remaining = pop_separator(remaining, ':')?;
    let mut states: Vec<&str> = Vec::with_capacity(state_var_count as usize);
    let mut line_consumed: usize = line.len() - remaining.len();

    for _ in 0..state_var_count {
        let (
            state_var_length,
            remaining
        ) = parse_number::<usize>(&line[line_consumed..])?;

        let remaining = pop_separator(remaining, ':')?;
        let (
            state,
            remaining,
        ) = take_n_characters(remaining, state_var_length)?;

        states.push(state);
        line_consumed = line.len() - remaining.len();
    }

    return Ok((states, &line[line_consumed..]));
}

fn parse_class_name(line: &str) -> Result<(ParsedClassName, &str), ParsedLineError> {
    let (class_name, rest_of_string) = take_until_whitespace(line)?;

    if !class_name.contains(":") {
        return Ok((ParsedClassName {
            class_name,
            parent_id: None,
            parent_class_name: None,
        }, rest_of_string));
    }

    let (parent_class, class_name) = take_until(class_name, ':')?;

    let rest_of_class_name = pop_separator(class_name, ':')?;
    let (parent_id, rest_of_class_name) = parse_number::<i32>(rest_of_class_name)?;
    let rest_of_class_name = pop_separator(rest_of_class_name, ':')?;
    let (class_name, _) = take_until_whitespace(rest_of_class_name)?;

    return Ok((ParsedClassName {
        class_name,
        parent_id: Some(parent_id),
        parent_class_name: Some(parent_class),
    }, rest_of_string));
}

fn take_until_whitespace(string: &str) -> Result<(&str, &str), ParsedLineError> {
    let count = string.chars().take_while(|c| {
        return !c.is_whitespace();
    }).count();

    return Ok((&string[0..count], &string[count..]));
}

fn take_until(string: &str, character: char) -> Result<(&str, &str), ParsedLineError> {
    let count = string.chars().take_while(|c| {
        return c != &character;
    }).count();

    return Ok((&string[0..count], &string[count..]));
}

fn take_n_characters(string: &str, n: usize) -> Result<(&str, &str), ParsedLineError> {
    if string.len() < n {
        return Err(ParsedLineError::NotEnoughCharacters);
    }

    return Ok((&string[..n], &string[n..]));
}

fn pop_separator(string: &str, separator: char) -> Result<&str, ParsedLineError> {
    let count = string.chars().take_while(|c| {
        return c == &separator;
    }).count();

    if count != 1 {
        return Err(ParsedLineError::BadSeparator);
    }

    return Ok(&string[1..]);
}

fn parse_number<T: FromStr<Err = ParseIntError>>(string: &str) -> Result<(T, &str), ParseIntError> {
    let num_count = string.chars().take_while(|c| {
        return c.is_numeric();
    }).count();

    return Ok((string[..num_count].parse::<T>()?, &string[num_count..]));
}

/*
use serde_json::{Result, Value};
use std::iter::FromIterator;
use std::error::Error;
use std::fmt::{self, Debug};
use std::io::{self, BufRead};
B
#[derive(Debug)]
struct ParsedLine<'a> {
    id: u32,
    parent_id: Option<u32>,  // -1 = not there
    parent: Option<&'a str>, // -1 = not there
    class_name: String,
}

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

enum ParseState {
    Id,
    ClassName,
    State,
    Args,
    Done,
}

fn parse_class_name(class_name: &str) -> Result<(&str, u32, &str), ParsedLineError> {
    let items: Vec<&str> = class_name.split(":").collect();
    if items.len() != 3 {
        return Err(ParsedLineError::BadClassName);
    }

    let id = match items[1].parse::<u32>() {
        Ok(v) => v,
        Err(_) => {
            return Err(ParsedLineError::BadClassName);
        }
    };

    return Ok((items[0], id, items[2]));
}

#[derive(Debug, PartialEq)]
enum ParseStateState {
    Len,
    ItemLen,
    Item,
    Done,
}

impl Default for ParseStateState {
    fn default() -> Self { return ParseStateState::Len; }
}

#[derive(Debug, Default)]
struct ParsedState<'a> {
    state: ParseStateState,
    state_count: i32,
    state_idx: i32,
    parse_length: i32,
    parse_idx: i32,
    out: Vec<&'a str>,
}

impl<'a> ParsedState<'a> {
    fn new() -> Self {
        return ParsedState {
            state_count: -1,
            ..ParsedState::default()
        };
    }

    fn set_state_count(&mut self, amount: i32) {
        self.state_count = amount;
    }

    fn is_done(&self) -> bool {
        return self.state_idx == self.state_count;
    }
}

fn parse_number(num: &str, err: ParsedLineError) -> Result<u32, ParsedLineError> {
    return match num.parse::<u32>() {
        Ok(v) => Ok(v),
        Err(_) => Err(err),
    };
}

fn parse_number_by_character(current_num: &mut Vec<char>, byte: char) -> Result<Option<i32>, Box<dyn Error>> {
    return match byte {
        ':' => {
            let s = String::from_iter(&*current_num);
            Ok(Some(s.parse()?))
        }
        '0'..='9' => {
            current_num.push(byte);
            Ok(None)
        }
        _ => {
            Err(Box::new(ParsedLineError::BadNumber))
        }
    };
}

fn parse_state<'a>(state: &'a mut ParsedState, chunk: &'a str) -> Result<bool, ParsedLineError> {
    let mut agg: Vec<char> = vec![];

    // split :
    for c in chunk.chars() {
        match state.state {
            ParseStateState::Len => {
                let out = match parse_number_by_character(&mut agg, c).
                        map_err(|_| ParsedLineError::BadState)? {
                            Some(v) => v,
                            None => continue,
                        };

                state.state = ParseStateState::ItemLen;
                state.set_state_count(out);
                agg.clear();
            }
            ParseStateState::ItemLen => {
                let out = match parse_number_by_character(&mut agg, c).
                        map_err(|_| ParsedLineError::BadState)? {
                            Some(v) => v,
                            None => continue,
                        };

                state.state = ParseStateState::Item;
                state.parse_length = out;
                state.parse_idx = 0;
                agg.clear();
            }

            ParseStateState::Item => {
                agg.push(c);
                state.parse_idx += 1;

                if state.parse_idx == state.parse_length {
                    let state_arg: Value =
                        serde_json::from_str(&agg.iter().collect()).
                            map_err(|_| ParsedLineError::BadState)?;

                    // My knowledge fo iterators
                }
            }

            ParseStateState::Done => {}
        };
    }

    return Ok(state.is_done());
}

fn parse(_line: &str) -> Result<ParsedLine, ParsedLineError> {
    let mut id: u32 = 0;
    let mut parent: Option<&str> = None;
    let mut parent_id: Option<u32> = None;
    let mut class_name: String = "suck_my_className".to_string();
    let mut state = ParseState::Id;
    let mut args_state_obj = ParsedState::new();
    let mut parsed_state_obj = ParsedState::new();

    for chunk in _line.split_whitespace() {
        match state {
            ParseState::Id => {
                id = parse_number(chunk, ParsedLineError::BadId)?;
            }

            ParseState::ClassName => {
                class_name = chunk.to_string();
                state = ParseState::State;

                if class_name.contains(":") {
                    let vals = parse_class_name(chunk)?;
                    class_name = vals.2.to_string();
                    parent = Some(vals.0);
                    parent_id = Some(vals.1);
                }
            }

            ParseState::State => {
                if parse_state(&mut parsed_state_obj, chunk)? {
                    state = ParseState::Args;
                }
            }

            ParseState::Args => {
                if parse_state(&mut args_state_obj, chunk)? {
                    state = ParseState::Done;
                }
            }

            ParseState::Done => {
                return Err(ParsedLineError::BadLine);
            }
        }
    }

    // return Err(ParsedLineError::BadLine)
    return Ok(ParsedLine {
        id,
        parent,
        parent_id,
        class_name,
    });
}
*/
