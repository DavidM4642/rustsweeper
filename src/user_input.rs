pub struct ParsedUserInput {
    pub flagged: bool,
    pub row: usize,
    pub col: usize,
}

impl ParsedUserInput {
    pub const fn new(flagged: bool, row: usize, col: usize) -> Self {
        Self { flagged, row, col }
    }
}

pub fn parse_input(input: &str) -> Option<ParsedUserInput> {
    // get rid of whitespace
    let clean_input = input.trim().to_uppercase();
    let stripped_prefix = clean_input.strip_prefix("F ");

    // check if flagged and if so remove the "F "
    let flagged = stripped_prefix.is_some();
    let clean_input = stripped_prefix.unwrap_or(clean_input.as_str());
    let (row, col) = parse_coords(clean_input)?;

    Some(ParsedUserInput::new(flagged, row, col))
}

pub fn parse_coords(input: &str) -> Option<(usize, usize)> // (row, col)
{
    if input.len() != 2 {
        return None;
    }

    let chars: Vec<char> = input.chars().collect();
    let letter = chars[0];
    let number_char = chars[1];

    if !('A'..='I').contains(&letter) {
        return None;
    }
    // Converts row into its corresponding row number
    let row = (letter as u8 - b'A') as usize;

    // if number_char < '1' || number_char > '9' {
    if !('1'..='9').contains(&number_char) {
        return None;
    }
    // converts col into the right column number
    let col = (number_char as u8 - b'1') as usize;

    Some((row, col))
}
// split up parse_input into two parts: parse_flagged and parse_coords

#[cfg(test)]
mod tests {
    use super::*;

    // This test verifies that a valid input is parsed correctly
    #[test]
    fn test_parse_input() {
        let user_input = "F A1";
        let result = parse_input(user_input);
        assert!(result.is_some()); // is_some means it is valid

        let parsed = result.expect("should be valid");
        assert!(parsed.flagged); // assert that the position is flagged
        assert_eq!(parsed.row, 0); // row should be 0 (A)
        assert_eq!(parsed.col, 0); // col should be 0 (1)
    }

    // This test verifies that an *invalid* input is parsed correctly
    #[test]
    fn test_parse_input_invalid() {
        let user_input = "Z99";
        let result = parse_input(user_input);
        assert!(result.is_none()); // is_none means it is invalid
    }
}
