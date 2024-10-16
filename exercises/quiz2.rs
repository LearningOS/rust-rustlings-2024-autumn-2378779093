// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // Update the function signature to accept a vector of tuples.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // Initialize an empty vector to store the output.
        let mut output: Vec<String> = vec![];

        // Iterate over the input tuples.
        for (string, command) in input {
            // Apply the command to the string.
            let transformed_string = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => {
                    let mut result = string.clone();
                    for _ in 0..n {
                        result.push_str(&string); // Append the original string `n` times.
                    }
                    result
                }
            };
            // Store the transformed string in the output vector.
            output.push(transformed_string);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    // Import the necessary items to have `transformer` in scope.
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foofoo");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
