pub fn search_pattern(input_string: &str, pattern: &str) -> usize {
    let string_vec: Vec<char> = input_string.chars().collect();
    let pattern_vec: Vec<char> = pattern.chars().collect();

    let mut _input_index: usize = 0;        //To maintain index of Input ie String_Vec
    let mut pattern_index: usize = 0;        //To maintain index of pattern ie pattern_Vec
    let firstocc: usize;

    while _input_index != string_vec.len() && string_vec[_input_index] != '\0' {
        while _input_index != string_vec.len()
            && string_vec[_input_index] != pattern_vec[0]
            && string_vec[_input_index] != '\0'
        {
            _input_index += 1;
        }
        firstocc = _input_index;                    //To maintain first index of pattern found
        while pattern_index != pattern_vec.len()
            && string_vec[_input_index] == pattern_vec[pattern_index]
            && string_vec[_input_index] != '\0'
            && pattern_vec[pattern_index] != '\0'
        {
            _input_index += 1;
            if (pattern_index + 1) != pattern_vec.len() {
                pattern_index += 1;
            } else {
                return firstocc;        //To maintain first index of pattern found
            }
        }
        _input_index = firstocc + 1;
        break;
    }
    string_vec.len()            //If Nothing found return Length of String Given as input(Unary Operator(-1) is not possible here)
}
