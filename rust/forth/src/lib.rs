use std::collections::HashMap;

pub type Value = i32;

pub type ForthResult = Result<(), Error>;
#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::default(),
            words: (vec![("*".to_string(), "".to_string()),
                         ("/".to_string(), "".to_string()),
                         ("+".to_string(), "".to_string()),
                         ("-".to_string(), "".to_string()),
                         ("DUP".to_string(), "".to_string()),
                         ("DROP".to_string(), "".to_string()),
                         ("SWAP".to_string(), "".to_string()),
                         ("OVER".to_string(), "".to_string())]
                .into_iter()
                .collect::<HashMap<_, _>>()),
        }

    }

    pub fn format_stack(&self) -> String {
        self.stack.iter().map(|tkn| tkn.to_string()).collect::<Vec<_>>().join(" ")
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        // make sure we check for error cases when new words are not ; terminated
        if input.starts_with(":") && !input.contains(";") {
            return Err(Error::InvalidWord);
        }

        // if multiple new words separated by ; are present we need to split
        // them and process them accordingly
        let (new_words, others): (Vec<&str>, Vec<&str>) = input.split(";")
            .partition(|str| str.starts_with(":"));
        for new_word in new_words {
            if new_word.starts_with(":") {
                let mut tokens = new_word.split(|c: char| c.is_whitespace() || c.is_control())
                    .filter(|str| !str.is_empty())
                    .collect::<Vec<_>>();
                tokens.push(";");

                try!(self.check_for_new_word(&tokens));
            }

        }
        let others_joined = others.join(" ");
        let tokens = others_joined.split(|c: char| c.is_whitespace() || c.is_control())
            .filter(|str| !str.is_empty())
            .collect::<Vec<_>>();


        for tkn in tokens {
            let mut rslt = 0;
            let number = tkn.parse::<Value>();
            match tkn {
                "+" => {
                    if self.stack.is_empty() {
                        return Err(Error::StackUnderflow);
                    }
                    rslt = self.stack.iter().fold(0, |mut acc, &x| {
                        acc += x;
                        acc
                    });
                    self.stack.clear();
                    self.stack.push(rslt);
                }
                "-" => {
                    if self.stack.is_empty() {
                        return Err(Error::StackUnderflow);
                    }
                    let first_elem = *self.stack.first().unwrap();
                    rslt = self.stack.iter().skip(1).fold(first_elem, |mut acc, &x| {
                        acc -= x;
                        acc
                    });
                    self.stack.clear();
                    self.stack.push(rslt);
                }
                "*" => {
                    if self.stack.is_empty() {
                        return Err(Error::StackUnderflow);
                    }
                    rslt = self.stack.iter().fold(1, |mut acc, &x| {
                        acc *= x;
                        acc
                    });
                    self.stack.clear();
                    self.stack.push(rslt);
                }
                "/" => {
                    if self.stack.is_empty() {
                        return Err(Error::StackUnderflow);
                    }
                    let first_elem = *self.stack.first().unwrap();
                    let mut division_by_zero = Ok(());
                    // special case when there's only one element in the stack
                    // and is zero
                    if self.stack.len() == 1 && first_elem == 0 {
                        return Err(Error::DivisionByZero);
                    }
                    rslt = self.stack.iter().skip(1).fold(first_elem, |mut acc, &x| {
                        if x == 0 {
                            division_by_zero = Err(Error::DivisionByZero);
                        }
                        acc /= x;
                        acc
                    });
                    self.stack.clear();
                    self.stack.push(rslt);
                }
                _ if number.is_ok() => self.stack.push(number.unwrap()),
                _ if self.words.get(&tkn.to_uppercase()).is_some() => {
                    let word =
                        self.words.get(&tkn.to_uppercase()).unwrap_or(&String::new()).clone();
                    if !word.is_empty() {
                        try!(self.eval(&word));
                    } else {
                        try!(self.apply_word_to_stack(&tkn.to_uppercase()));
                    }
                }
                _ => return Err(Error::UnknownWord),

            }
        }

        Ok(())
    }

    fn apply_word_to_stack(&mut self, word: &str) -> ForthResult {
        match word {
            "DUP" => {
                if self.stack.is_empty() {
                    return Err(Error::StackUnderflow);
                }
                let last = self.stack[self.stack.len() - 1];
                self.stack.push(last);
            }
            "DROP" => {
                if self.stack.is_empty() {
                    return Err(Error::StackUnderflow);
                }
                self.stack.pop();
            }
            "SWAP" => {
                if self.stack.len() < 2 {
                    return Err(Error::StackUnderflow);
                }
                let idx1 = self.stack.len() - 1;
                let idx2 = self.stack.len() - 2;

                self.stack.swap(idx1, idx2);
            }
            "OVER" => {
                if self.stack.len() < 2 {
                    return Err(Error::StackUnderflow);
                }
                let idx = self.stack.len() - 2;
                let over_elem = self.stack[idx];

                self.stack.push(over_elem);

            }
            _ => {}
        }
        Ok(())
    }

    fn check_for_new_word(&mut self, word: &Vec<&str>) -> ForthResult {
        if word.last() != Some(&";") {
            return Err(Error::InvalidWord);
        }

        if word.len() < 4 {
            return Err(Error::InvalidWord);
        }

        if let Ok(_) = word[1].parse::<Value>() {
            return Err(Error::InvalidWord);
        }

        self.words.insert(word[1].to_string().to_uppercase(),
                          word.iter()
                              .cloned()
                              .skip(2)
                              .take_while(|tkn| *tkn != ";")
                              .collect::<Vec<_>>()
                              .join(" "));

        Ok(())
    }
}
