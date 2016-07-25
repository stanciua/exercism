pub struct WordProblem {
    command: String,
}
enum Operation {
    Plus,
    Minus,
    Division,
    Multiplication,
    RaisedTo,
}
impl WordProblem {
    pub fn new(command: &str) -> WordProblem {
        WordProblem { command: command.to_owned() }
    }
    pub fn answer(&self) -> Result<i32, String> {
        let mut op_str: String = self.command
                                     .chars()
                                     .skip_while(|ch| !ch.is_digit(10) && *ch != '-')
                                     .collect();
        op_str = op_str.trim_right_matches('?').to_owned();

        if op_str.len() == 0 {
            return Err("invalid command, no operations!".to_owned());
        }

        let operations = op_str.split_whitespace()
                               .filter(|tkn| {
                                   *tkn != "power" && *tkn != "by" && *tkn != "to" && *tkn != "the"
                               })
                               .map(|tkn| {
                                   if tkn.ends_with("th") {
                                       tkn.trim_right_matches("th")
                                   } else {
                                       tkn
                                   }
                               })
                               .collect::<Vec<_>>();

        if let Some(tkn) = operations.iter().last() {
            if !tkn.chars().all(|ch| ch.is_digit(10) || ch == '-') {
                return Err("command is not complete, it should end in a digit!".to_owned());
            }
        }


        let mut total = 0i32;
        let mut cur_op = Operation::Plus;

        for token in operations {
            match token {
                "plus" => cur_op = Operation::Plus,
                "minus" => cur_op = Operation::Minus,
                "divided" => cur_op = Operation::Division,
                "multiplied" => cur_op = Operation::Multiplication,
                "raised" => cur_op = Operation::RaisedTo,
                _ => {
                    let number = try!(token.parse::<i32>().map_err(|err| err.to_string()));
                    total = WordProblem::apply_op(total, number, &cur_op);
                }
            }
        }
        Ok(total)
    }
    fn apply_op(total: i32, cur_num: i32, op: &Operation) -> i32 {
        match *op {
            Operation::Plus => total + cur_num,
            Operation::Minus => total - cur_num,
            Operation::Multiplication => total * cur_num,
            Operation::Division => total / cur_num,
            Operation::RaisedTo => total.pow(cur_num as u32),
        }
    }
}
