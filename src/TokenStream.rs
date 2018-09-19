use InputStream::InputStream;
use regex::Regex;

pub struct TokenStream{
    pub pos: i32,
    pub line: i32,
    pub col: i32,
    pub input: InputStream,
    pub keywords: [String; 3]
}

impl TokenStream {
    pub fn read_next(&self, input: &InputStream) {
        self.read_while();
        // self.read_while("".to_string());
        if input.eof() { () };
        let ch = input.peek();
        
        // Comment handler
        if ch == '#' {
            // self.skip_comment();
            // return self.read_next();
        }
        
        // String Handler
        if ch == '"' {
            // return self.read_string();
        }
        
        // // Numerical Handler
        if self.is_digit(ch) {
        //     // return self.read_number();
        }
        
        // // Identifier Handler
        // if self.is_id_start(ch) {
        //     // return self.read_ident();
        // }
        
        // // Punctuation Handler
        // if self.is_punc(ch) {
        //     return Token {
        //         dataType: "punc".to_string(),
        //         value: input.next()
        //     }
        // }
        
        // // Operation Handler
        // if self.is_op_char(ch) {
        //     return Token {
        //         dataType: "op".to_string(),
        //         value: "".to_string()
        //         // value: self.read_while(self.is_op_char)
        //     }
        // }
        
        // input.croak("Can't handle character: ".to_string() + ch);
        
        ()
    }
    
    pub fn is_keyword(&self, ch: String) -> bool {
        // return self.keywords
        let index = self.keywords.iter().position(|ref s| s.to_string() == ch);
        return index >= Some(0);
        // return true;
    }
    
    pub fn is_digit(&self, ch: char) -> bool {
        let reg = Regex::new(r"[0-9]").unwrap();
        return reg.is_match(&ch.to_string());
        // return x.is_numeric();
    }
    
    pub fn is_id_start(&self, ch: char) -> bool {
        let reg = Regex::new(r"[a-zλ_]").unwrap();
        return reg.is_match(&ch.to_string());
        // let reg = Regex::new(r"/[")
    }
    
    pub fn is_id(&self, ch: char) -> bool {
        // TODO: Implement or
        return self.is_id_start(ch);
    }
    
    pub fn is_op_char(ch: char) -> bool {
        let reg = Regex::new(r"[\+\-\*\/\%\=\&\|\<\>\!]").unwrap();
        return reg.is_match(&ch.to_string());
    }

    pub fn is_punc(ch: char) -> bool {
        let reg = Regex::new(r"[\,\;\(\)\{\}\[\]]").unwrap();
        return reg.is_match(&ch.to_string());
    }
    
    pub fn is_whitespace(&self, ch: char) -> bool {
        let reg = Regex::new(r"[\t\n]").unwrap();
        return reg.is_match(&ch.to_string());
    }
    
    
    // Predicate is a function of sorts
    
    pub fn read_while(&self) -> String {
        let retval = "";
        //TODO: Add in checking of the predicate with input.peek
        while !self.input.eof() {
            // retval += input.next();
        }
        return retval.to_string();
    }
    
    pub fn read_number() -> Token {
        let mut has_dot = false;
        let mut number = 0;
        // TODO: Add in read while
        // let mut number = read_while()
        return Token {
            data_type: "num".to_string(),    
            value: number.to_string(),
        }
    }
    
    pub fn read_ident() {
        let mut id = 0;
        // return Token {
        //     data_type: is_keyword(id)
        // }
    }
    
    pub fn read_escaped(&self, end: char) -> &str {
        let mut escaped = false;
        let retval = "";
        while !self.input.eof() {
            let ch = '\\';
            // let ch = self.input.next();
            if escaped {
                // str += ch;
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == end {
                break;
            } else {
                // str += ch;
            }
        }
        return retval;
    }
    
    pub fn read_string(&self) -> Token {
        return Token {
            data_type: "str".to_string(),
            value: self.read_escaped('"').to_string()
        };
    }
    

    // pub fn skip_comment() {
        
    // }
    
    // pub fn peek() {
        
    // }
    
    // pub fn next() {
        
    // }
    
    // pub fn eof() {
        
    // }
}

pub struct Token {
    data_type: String,
    value: String
}