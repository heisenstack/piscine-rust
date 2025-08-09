use logic_number::*;
fn main() {
    let array = [9, 10, 153, 154];
    for pat in &array {
        if number_logic(*pat) == true {
            println!(
                "this number returns {} because the number {} obey the rules of the sequence",
                number_logic(*pat),
                pat
            )
        }
        if number_logic(*pat) == false {
            println!("this number returns {} because the number {} does not obey the rules of the sequence", number_logic(*pat),pat )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert!(number_logic(0))
    }

    #[test]
    fn test_single_digit_numbers() {
        assert!(number_logic(5));
        assert!(number_logic(9))
    }

    #[test]
    fn test_two_digit_numbers() {
        assert!(!number_logic(10))
    }

    #[test]
    fn test_three_or_more_digit_number() {
        assert!(number_logic(153));
        assert!(!number_logic(100));
        assert!(number_logic(9474));
        assert!(!number_logic(9475));
        assert!(number_logic(9_926_315));
        assert!(!number_logic(9_926_316))
    }
}