use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

const BASE_TOKENS: &[&str] = &["+", "-", "*", "/", "DUP", "OVER", "DROP", "SWAP"];

fn is_base_token(word: &str) -> bool {
    BASE_TOKENS.contains(&word)
}

pub struct Forth {
    stack: Vec<Value>,
    user_operations: HashMap<String, String>,
    included_keys: HashMap<String, Vec<String>>
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
        Forth { stack: Vec::new(), user_operations: HashMap::new(), included_keys: HashMap::new() }
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
            _ if self.user_operations.contains_key(operation_name) =>
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
                        self.change_included_word_if_need(&key);
                    } else {
                        return Err(Error::InvalidWord);
                    }
                } else if self.user_operations.contains_key(&tmp_word) {
                    words.push_str(&tmp_word);
                    self.add_included_word(tmp_word.clone(), key.clone());
                    words.push(' ');
                } else if is_base_token(&tmp_word) || tmp_word.parse::<Value>().is_ok() {
                    words.push_str(&tmp_word);
                    words.push(' ');
                } else {
                    return Err(Error::UnknownWord);
                }
                tmp_word.clear();
            } else if ch == ';' {
                words.remove(words.len() -1);
                break;
            } else {
                tmp_word.push(ch.to_ascii_uppercase()); // TODO read inplace instead 
            }
        }

        self.user_operations.insert(key, words);

        Ok(())
    }
    
    fn eval_user_operation(&mut self, operation_name: &str) -> std::result::Result<(), Error> {
        if let Some(command) =  self.user_operations.get(operation_name).cloned() {
            self.eval(&command)
        } else {
            Ok(())
        }
    }
    
    fn add_included_word(&mut self, included_word: String, word: String) {
        let e = self.included_keys.entry(included_word).or_insert(Vec::new());
        e.push(word);
    }
    
    fn change_included_word_if_need(&mut self, key: &str) {
        if let Some(used_keys) = self.included_keys.remove(key) {
            let value = self.user_operations.remove(key).unwrap();
            for used_key in used_keys {
                let modified_value = self.user_operations.get(&used_key).unwrap()
                    .replace(key, &value);
                self.user_operations.insert(used_key, modified_value);
            }
        }
    }

}
