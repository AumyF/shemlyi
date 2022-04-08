use std::str::FromStr;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tk = Tokenizer::new(input);

    while !tk.at_eof() {
        tk.skip_space();
        if !(tk.operator("+")
            || tk.operator("-")
            || tk.operator("(")
            || tk.operator(")")
            || tk.operator("[")
            || tk.operator("]")
            || tk.operator("{")
            || tk.operator("}")
            || tk.operator("<=")
            || tk.operator(">=")
            || tk.operator("<|")
            || tk.operator("<")
            || tk.operator(">")
            || tk.operator("=")
            || tk.operator(":")
            || tk.operator(",")
            || tk.operator("if")
            || tk.operator("then")
            || tk.operator("let")
            || tk.operator("def")
            || tk.identifier()
            || tk.integer())
        {
            panic!("Unknown symbol: {}", tk.get_input())
        }
    }

    tk.tokens.push(Token::EOF);

    tk.tokens
}

/// Given a string, parses it and returns the result and its length.
fn str_to_fromstr<F: FromStr>(str: &str) -> Result<(F, usize), F::Err> {
    let index = str
        .bytes()
        .position(|byte| !byte.is_ascii_digit())
        .unwrap_or(str.len());

    let digit_part = &str[..index];

    digit_part.parse().map(|value| (value, index))
}

struct Tokenizer {
    input: String,
    position: usize,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(input: impl Into<String>) -> Tokenizer {
        let input: String = input.into();

        Tokenizer {
            input,
            position: 0,
            tokens: Vec::new(),
        }
    }

    fn get_input(&self) -> &str {
        &self.input[self.position..]
    }

    fn increment(&mut self, count: usize) {
        self.position = self.position + count;
    }
    fn at_eof(&self) -> bool {
        self.get_input().is_empty()
    }
    fn skip_space(&mut self) {
        let input = self.get_input();
        if input.starts_with(" ") {
            let len = input
                .bytes()
                .position(|byte| byte != b' ')
                .unwrap_or(input.len());

            self.increment(len);
        }
    }

    fn operator(&mut self, op: &str) -> bool {
        let input = self.get_input();

        if input.starts_with(op) {
            let op_len = op.len();
            let token = Token::Operator(input[..op_len].to_string());

            self.tokens.push(token);
            self.increment(op_len);

            true
        } else {
            false
        }
    }
    fn integer(&mut self) -> bool {
        let input = self.get_input();

        if let Ok((num, len)) = str_to_fromstr(input) {
            self.tokens.push(Token::IntegerLiteral(num));
            self.increment(len);

            true
        } else {
            false
        }
    }
    fn identifier(&mut self) -> bool {
        let input = self.get_input();

        let mut chars = input.chars().peekable();
        let first = chars.peek();

        match first {
            Some(char) if char.is_alphabetic() => {
                let ident: String = chars.take_while(|char| char.is_alphanumeric()).collect();
                let len = ident.len();

                self.tokens.push(Token::Identifier(ident));
                self.increment(len);

                true
            }
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    IntegerLiteral(u32),
    Operator(String),
    Identifier(String),
    EOF,
}

impl Token {
    fn operator(str: &str) -> Token {
        Token::Operator(str.to_string())
    }
    fn identifier(str: &str) -> Token {
        Token::Identifier(str.to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn operator_test() {
        use Token::*;

        let tokens = tokenize("66+34");
        assert_eq!(
            tokens,
            vec![
                IntegerLiteral(66),
                Operator("+".to_string()),
                IntegerLiteral(34),
                EOF
            ]
        );

        let tokens = tokenize(r#"def add(x: Int, y: Int) = x + y"#);
        assert_eq!(
            tokens,
            vec![
                Token::operator("def"),
                Token::identifier("add"),
                Token::operator("("),
                Token::identifier("x"),
                Token::operator(":"),
                Token::identifier("Int"),
                Token::operator(","),
                Token::identifier("y"),
                Token::operator(":"),
                Token::identifier("Int"),
                Token::operator(")"),
                Token::operator("="),
                Token::identifier("x"),
                Token::operator("+"),
                Token::identifier("y"),
                Token::EOF
            ]
        );
    }

    #[test]
    fn strto_test() {
        assert_eq!(str_to_fromstr("34"), Ok((34, 2)));
        assert_eq!(str_to_fromstr("123"), Ok((123, 3)));
        assert_eq!(str_to_fromstr("123あいう"), Ok((123, 3)));
        assert!(str_to_fromstr::<i32>("あbc").is_err());
        assert!(str_to_fromstr::<i32>("").is_err());
    }
}
