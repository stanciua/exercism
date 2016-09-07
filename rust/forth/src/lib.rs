use std::collections::HashMap;

pub type Value = i32;

pub type ForthResult = Result<(), Error>;
#[derive(Default)]
pub struct Forth {
    stack: Vec<Value>,
    word_table: HashMap<String, String>,
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
            word_table: (vec![("*".to_string(), "".to_string()),
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

    fn get_words_and_non_words_from_input(input: &str)
                                          -> Result<(Vec<Vec<&str>>, Vec<&str>), Error> {
        let tokens = input.split(|c: char| c.is_whitespace() || c.is_control())
            .filter(|str| !str.is_empty())
            .collect::<Vec<_>>();

        let mut inside_new_word = false;
        let mut new_word = Vec::new();
        let mut non_new_words = Vec::new();
        let mut new_words = Vec::new();

        for tkn in tokens {
            match tkn {
                ":" => {
                    new_word.push(tkn);
                    inside_new_word = true;
                }
                ";" => {
                    if !inside_new_word {
                        return Err(Error::InvalidWord);
                    }
                    new_word.push(tkn);
                    new_words.push(new_word.clone());
                    inside_new_word = false;
                }
                _ if inside_new_word => {
                    new_word.push(tkn);
                }
                _ => {
                    non_new_words.push(tkn);
                }
            }
        }

        if inside_new_word {
            return Err(Error::InvalidWord);
        }

        Ok((new_words, non_new_words))

    }

    pub fn eval(&mut self, input: &str) -> ForthResult {

        let (new_words, non_new_words) = try!(Forth::get_words_and_non_words_from_input(input));

        for new_word in new_words {
            try!(self.validate_and_insert_new_word_into_table(&new_word));
        }
        for tkn in non_new_words {
            let number = tkn.parse::<Value>();
            match tkn {
                "+" => {
                    if self.stack.is_empty() {
                        return Err(Error::StackUnderflow);
                    }
                    let rslt = self.stack.iter().fold(0, |mut acc, &x| {
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
                    let rslt = self.stack
                        .iter()
                        .skip(1)
                        .fold(*self.stack.first().unwrap(), |mut acc, &x| {
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
                    let rslt = self.stack.iter().fold(1, |mut acc, &x| {
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
                    let rslt = self.stack.iter().skip(1).fold(first_elem, |mut acc, &x| {
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
                _ if self.word_table.get(&tkn.to_uppercase()).is_some() => {
                    let word =
                        self.word_table.get(&tkn.to_uppercase()).unwrap_or(&String::new()).clone();
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

    fn validate_and_insert_new_word_into_table(&mut self, word: &[&str]) -> ForthResult {
        if word.len() < 4 {
            return Err(Error::InvalidWord);
        }

        if let Ok(_) = word[1].parse::<Value>() {
            return Err(Error::InvalidWord);
        }

        self.word_table.insert(word[1].to_string().to_uppercase(),
                               word.iter()
                                   .cloned()
                                   .skip(2)
                                   .take_while(|tkn| *tkn != ";")
                                   .collect::<Vec<_>>()
                                   .join(" "));

        Ok(())
    }
}
