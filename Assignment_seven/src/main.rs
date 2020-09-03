use assignment_seven::string_processing::pattern_searching::search_pattern;
use assignment_seven::string_processing::substring_generation::substring;


fn main() {
    let input_string: String = String::from("Pankaj Chaudhary");
    let pattern: String = String::from("Cha");
    let lowercase_input: String = input_string.to_lowercase();
    let lowercase_pattern: String = pattern.to_lowercase();
    let index= search_pattern(&lowercase_input, &lowercase_pattern);
    println!("{} found at index {}", pattern, index);

    let input_str = "pankaj";
    substring(&input_str);
}
