fn transform_to_haiku(s: String) -> Option<String> {
    let words: Vec<&str> = s.split_whitespace().collect();
    let mut transformed_haiku = String::new();
    let mut syllable_counts = vec![0; 3]; // syllable counts for each line
    let mut line_index = 0; // current line index

    for word in words {
        let syllables = count_syllables_in_word(word);
        syllable_counts[line_index] += syllables;
        transformed_haiku.push_str(word);
        transformed_haiku.push(' ');

        // If we have already counted 5 or more syllables for the current line,
        // we move to the next line to start counting again
        if syllable_counts[line_index] == 5 && line_index == 0 {
            transformed_haiku.push('\n');
            line_index += 1;
        }

        if syllable_counts[line_index] == 7 && line_index == 1 {
            transformed_haiku.push('\n');
            line_index += 1;
        }

        if syllable_counts[line_index] == 5 && line_index == 2 {
            transformed_haiku.push('\n');
            return Some(transformed_haiku);
        }
        if line_index > 2 {
            // If more than 3 lines are formed, it's not possible to transform into a haiku
            return None;
        }
    }

    // Check if we have a valid haiku, if not return None
    if syllable_counts != [5, 7, 5] {
        return None;
    }

    Some(transformed_haiku)
}

fn count_syllables_in_word(word: &str) -> usize {
        let mut syllable_count = 0;
        let mut prev_vowel = false;
    
        for c in word.chars() {
            if "aeiouyAEIOUYуеыаэояиюУЕЫАОЭЯИЮ".contains(c) {
                // If current character is a vowel
                if !prev_vowel {
                    // If previous character was not a vowel (i.e., consecutive vowels)
                    syllable_count += 1;
                }
                prev_vowel = true;
            } else {
                prev_vowel = false;
            }
        }
    
        syllable_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_to_haiku_valid() {
        let input = "one one on one one one on one one on".to_string();
        let expected_output = "one one on \none one one on \none one on \n".to_string();
        assert_eq!(transform_to_haiku(input), Some(expected_output));
    }
    
    #[test]
    fn test_transform_to_haiku_invalid_2() {
        let input = "A Friday Haiku Time to celebrate freedom to create my art.".to_string();
        let expected_output = "A Friday Haiku \nTime to celebrate freedom \nto create my art. \n".to_string();
        assert_eq!(transform_to_haiku(input), None);
    }
    #[test]
    fn test_transform_to_haiku_valid_3() {
        let input = "Ты улыбнулась. С медленной льдины вдали Птица взлетает.".to_string();
        let expected_output = "Ты улыбнулась. \nС медленной льдины вдали \nПтица взлетает. \n".to_string();
        assert_eq!(transform_to_haiku(input), Some(expected_output));
    }

    #[test]
    fn test_transform_to_haiku_invalid() {
        let input = "one one on one one one on one".to_string();
        assert_eq!(transform_to_haiku(input), None);
    }
    #[test]
    fn test_count_syllables_in_word() {
        assert_eq!(count_syllables_in_word("hello"), 2);
        assert_eq!(count_syllables_in_word("world"), 1);
        assert_eq!(count_syllables_in_word("haiku"), 2);
    }

}

fn main() {
    let input = "Ты улыбнулась. С медленной льдины вдали Птица взлетает.".to_string();
    match transform_to_haiku(input) {
        Some(haiku) => println!("{}", haiku),
        None => println!("The input cannot be transformed into a haiku"),
    }
}