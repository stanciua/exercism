use std::mem;

pub struct Brackets {
    brackets: String,
}

impl Brackets {
    pub fn from(input: &str) -> Brackets {
        // discard any character that is not a brace or bracket
        Brackets {
            brackets: input.chars()
                .filter(|&c| c == '[' || c == ']' || c == '{' || c == '}' || c == '(' || c == ')')
                .collect(),
        }
    }

    pub fn are_balanced(&mut self) -> bool {
        let mut old_brackets = mem::replace(&mut self.brackets, String::new());
        let mut rest_brackets = old_brackets.replace("[]", "").replace("{}", "").replace("()", "");
        while old_brackets != rest_brackets {
            old_brackets = rest_brackets;
            rest_brackets = old_brackets.replace("[]", "").replace("{}", "").replace("()", "");
        }

        rest_brackets.is_empty()
    }
}
