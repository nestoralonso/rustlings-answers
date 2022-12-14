// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// Execute `rustlings hint quiz2` or use the `hint` watch subcommand for a hint.

#[derive(PartialEq, Debug)]
pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            output.push(
                apply_cmd(&string, &command)
            );
        }
        output
    }

    // create a functions that takes a tuple of two strings and returns a string
    // the first string is the input string, the second string is the command
    // the function should return the input string with the command applied
    pub fn apply_cmd(st: &str, command: &Command) -> String {
        let res = match command {
            Command::Uppercase => st.to_uppercase().to_string(),
            Command::Trim => st.trim().to_string(),
            Command::Append(times) => st.to_owned() + &"bar".repeat(*times),
        };

        res.to_string()
    }
}

#[cfg(test)]
mod tests {
    // TODO: What to we have to import to have `transformer` in scope?
    use my_module::transformer;

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
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
