// Return the array if string from the function which contain all the substrings.
pub fn substring(input_str: &str) {
    let string_vec: Vec<char> = input_str.chars().collect();
    let n: usize = string_vec.len();
    let len: usize = 1;

    for len in len..=n {
        let i: usize = 0;

        for i in i..=n - len {
            let j: usize = i + len - 1;
            let print_char: usize = i;

            for print_char in print_char..=j {
                print!("{}", string_vec[print_char]);
            }
            print!(" ");
        }
    }
}