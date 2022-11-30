// Santa is trying to deliver presents in a large apartment building.
// '(' means he should go one floor up
// ')' means he should go one floor down

// To what floor do the instructions take Santa?
pub fn process_paranthesis(input: String) -> i64 {
    input
        .chars()
        .into_iter()
        .map(|x| {
            if x == '(' {
                1
            } else if x == ')' {
                -1
            } else {
                0
            }
        })
        .reduce(|accum: i64, item: i64| accum + item)
        .unwrap_or_default()
}

// At what point (character position) does Santa enter the basement (floor -1)?
pub fn basement_entered(input: String) -> i64 {
    let instructions = input
        .chars()
        .into_iter()
        .map(|x| {
            if x == '(' {
                1
            } else if x == ')' {
                -1
            } else {
                0
            }
        })
        .enumerate();

    let mut sum: i64 = 0;
    for instruction in instructions {
        sum += instruction.1;

        if sum == -1 {
            // Floor count starts at 1
            return instruction.0.try_into().unwrap_or(0) + 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_parenthesis_one() {
        let expected = 0;
        let actual1 = process_paranthesis(String::from("(())"));
        let actual2 = process_paranthesis(String::from("()()"));
        assert_eq!(expected, actual1);
        assert_eq!(expected, actual2);
    }

    #[test]
    fn test_process_parenthesis_two() {
        let expected = 3;
        let actual1 = process_paranthesis(String::from("((("));
        let actual2 = process_paranthesis(String::from("(()(()("));
        let actual3 = process_paranthesis(String::from("))((((("));

        assert_eq!(expected, actual1);
        assert_eq!(expected, actual2);
        assert_eq!(expected, actual3);
    }

    #[test]
    fn test_process_parenthesis_three() {
        let expected = -1;
        let actual1 = process_paranthesis(String::from("())"));
        let actual2 = process_paranthesis(String::from("))("));

        assert_eq!(expected, actual1);
        assert_eq!(expected, actual2);
    }

    #[test]
    fn test_process_parenthesis_four() {
        let expected = -3;
        let actual1 = process_paranthesis(String::from(")))"));
        let actual2 = process_paranthesis(String::from(")())())"));

        assert_eq!(expected, actual1);
        assert_eq!(expected, actual2);
    }

    #[test]
    fn test_basement_entered_one() {
        let expected = 1;
        let actual = basement_entered(String::from(")"));

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_basement_entered_two() {
        let expected = 5;
        let actual = basement_entered(String::from("()())"));

        assert_eq!(expected, actual)
    }
}
