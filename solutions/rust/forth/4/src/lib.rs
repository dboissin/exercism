use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

const BASE_TOKENS: &[&str] = &["+", "-", "*", "/", "DUP", "OVER", "DROP", "SWAP"];
const IDX_PREFIX: char = '@';

fn is_base_token(word: &str) -> bool {
    BASE_TOKENS.contains(&word)
}

pub struct Forth {
    stack: Vec<Value>,
    user_operations_idx: HashMap<String, usize>,
    user_operations: Vec<String>,
    seq_id: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            user_operations_idx: HashMap::new(),
            user_operations: Vec::new(),
            seq_id :0
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        if input.starts_with(':') {
            return self.add_user_operation(input);
        }
        for el in input.split_whitespace() {
            if let Ok(number) = el.parse::<Value>() {
                self.stack.push(number);
            } else {
                self.try_exec_operation(el)?;
            }
        }
        Ok(())
    }

    fn try_exec_operation(&mut self, el: &str) -> Result {
        let length = self.stack.len();
        let operation_name_upper = el.to_ascii_uppercase();
        let operation_name = operation_name_upper.as_str();

        let min_length_need = match operation_name {
            _ if operation_name.starts_with(IDX_PREFIX) =>
                return self.eval_internal_operation(operation_name),
            _ if self.user_operations_idx.contains_key(operation_name) =>
                return self.eval_user_operation(operation_name),
            "+" | "-" | "*" | "/" | "OVER" | "SWAP" => 2,
            "DUP" | "DROP" => 1,
            _ => return Err(Error::UnknownWord)
        };
        if length < min_length_need {
            return  Err(Error::StackUnderflow);
        }

        let (last_element, last_idx) = if min_length_need == 2 && operation_name != "OVER" {
            (self.stack.pop(), length - 2)
        } else {
            (None, length - 1)
        };
        match operation_name {
            "+" => self.stack[last_idx] += last_element.unwrap(),
            "-" => self.stack[last_idx] -= last_element.unwrap(),
            "*" => self.stack[last_idx] *= last_element.unwrap(),
            "/" => {
                if last_element.unwrap() == 0 {
                    return Err(Error::DivisionByZero);
                }
                self.stack[last_idx] /= last_element.unwrap()
            },
            "DUP" => self.stack.push(self.stack[last_idx]),
            "OVER" => self.stack.push(self.stack[length-2]),
            "DROP" => _ = self.stack.pop(),
            "SWAP" => {
                let tmp = self.stack[last_idx];
                self.stack[last_idx] = last_element.unwrap();
                self.stack.push(tmp);
            },
            _ => return Err(Error::UnknownWord)
        };

        Ok(())
    }
    
    fn add_user_operation(&mut self, input: &str) -> Result {
        let mut key = String::new();
        let mut words = String::new();
        let mut tmp_word = String::new();

        for ch in input[2..].chars() {
            if ch.is_ascii_whitespace() {
                if key.is_empty() {
                    if tmp_word.parse::<Value>().is_err() {
                        key.push_str(&tmp_word);
                    } else {
                        return Err(Error::InvalidWord);
                    }
                } else {
                    if let Some(idx) = self.user_operations_idx.get(&tmp_word) {
                        words.push_str(&format!("{IDX_PREFIX}{idx}"));
                    } else if is_base_token(&tmp_word) || tmp_word.parse::<Value>().is_ok() {
                        words.push_str(&tmp_word);
                    } else {
                        return Err(Error::UnknownWord);
                    }
                    words.push(' ');
                }
                tmp_word.clear();
            } else if ch == ';' {
                words.remove(words.len() -1);
                break;
            } else {
                tmp_word.push(ch.to_ascii_uppercase());
            }
        }

        self.insert_user_operation(key, words)
    }

    fn insert_user_operation(&mut self, key: String, words: String) -> Result {
        self.user_operations.push(words);
        self.user_operations_idx.insert(key, self.seq_id);
        self.seq_id += 1;
        Ok(())
    }
    
    fn eval_user_operation(&mut self, operation_name: &str) -> Result {
        if let Some(command) =  self.user_operations_idx.get(operation_name) {
            self.eval(&self.user_operations[*command].clone())
        } else {
            Ok(())
        }
    }

    fn eval_internal_operation(&mut self, operation_name: &str) -> Result {
        let idx = operation_name[1..].parse::<usize>().unwrap();
        self.eval(&self.user_operations[idx].clone())
    }

}
