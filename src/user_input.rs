pub struct ParsedUserInput {
    pub flagged: bool,
    pub row: usize,
    pub col: usize,
}
pub fn parse_input(input: &str) -> Option<ParsedUserInput> {
    // get rid of whitespace
    let clean_input = input.trim().to_uppercase();
    // check if flagged and if so remove the "F "
    let mut flagged = false;
    let coords = clean_input
        .strip_prefix("F ")
        .map_or(clean_input.as_str(), |stripped| {
            flagged = true;
            stripped
        });

    // confirm lenths of coords
    if coords.len() != 2 {
        return None;
    }

    let chars: Vec<char> = coords.chars().collect();
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

    Some(ParsedUserInput { flagged, row, col })
}

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
