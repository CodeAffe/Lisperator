extern crate regex;
use regex::Regex;


fn main() {
    let mut input = InputStream {
        pos: 0,
        line: 1,
        col: 0,
        input: "t\nest".to_string().chars().collect()
    };
    
    let mut token = TokenStream {
        pos: 0,
        line: 1,
        col: 0,
        input: input,
        keywords: ["if".to_string(), "then".to_string(), "else".to_string()]
    };
    
    let retrun = token.is_digit('1');
    
    println!("Return: {}", retrun);
}

struct TokenStream{
    pos: i32,
    line: i32,
    col: i32,
    input: InputStream,
    keywords: [String; 3]
}

impl TokenStream {
    fn read_next(&self, input: &InputStream) {
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
    
    fn is_keyword(&self, ch: String) -> bool {
        // return self.keywords
        let index = self.keywords.iter().position(|ref s| s.to_string() == ch);
        return index >= Some(0);
        // return true;
    }
    
    fn is_digit(&self, ch: char) -> bool {
        let reg = Regex::new(r"[0-9]").unwrap();
        return reg.is_match(&ch.to_string());
        // return x.is_numeric();
    }
    
    fn is_id_start(&self, ch: char) -> bool {
        let reg = Regex::new(r"[a-zλ_]").unwrap();
        return reg.is_match(&ch.to_string());
        // let reg = Regex::new(r"/[")
    }
    
    fn is_id(&self, ch: char) -> bool {
        // TODO: Implement or
        return self.is_id_start(ch);
    }
    
    fn is_op_char(ch: char) -> bool {
        let reg = Regex::new(r"[\+\-\*\/\%\=\&\|\<\>\!]").unwrap();
        return reg.is_match(&ch.to_string());
    }

    fn is_punc(ch: char) -> bool {
        let reg = Regex::new(r"[\,\;\(\)\{\}\[\]]").unwrap();
        return reg.is_match(&ch.to_string());
    }
    
    fn is_whitespace(&self, ch: char) -> bool {
        let reg = Regex::new(r"[\t\n]").unwrap();
        return reg.is_match(&ch.to_string());
    }
    
    
    // Predicate is a function of sorts
    
    fn read_while(&self) -> String {
        let retval = "";
        //TODO: Add in checking of the predicate with input.peek
        while !self.input.eof() {
            // retval += input.next();
        }
        return retval.to_string();
    }
    
    fn read_number() -> Token {
        let mut has_dot = false;
        let mut number = 0;
        // TODO: Add in read while
        // let mut number = read_while()
        return Token {
            data_type: "num".to_string(),    
            value: number.to_string(),
        }
    }
    
    fn read_ident() {
        let mut id = 0;
        // return Token {
        //     data_type: is_keyword(id)
        // }
    }
    
    fn read_escaped(&self, end: char) -> &str {
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
    
    fn read_string(&self) -> Token {
        return Token {
            data_type: "str".to_string(),
            value: self.read_escaped('"').to_string()
        };
    }
    

    // fn skip_comment() {
        
    // }
    
    // fn peek() {
        
    // }
    
    // fn next() {
        
    // }
    
    // fn eof() {
        
    // }
}

struct InputStream{
    pos: i32,
    line: i32,
    col: i32,
    input: Vec<char>
}

impl InputStream {
    /**
    * Returns the next value and discards it from stream
    */
    fn next(&mut self) -> char {
        let character = self.input[(self.pos+1) as usize];
        if character == '\n' {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        // character
        self.input[0]
    }

    // Returns the next value without removing it form the stream    
    fn peek(&self) -> char {
        self.input[self.pos as usize]
    }
    
    // Returns true if the stream is at the end
    fn eof(&self) -> bool {
        self.peek().to_string() == ""
    }
    
    // Throws an error
    fn croak(&self, msg: String) {
        panic!("{} ({}:{})", msg, self.line, self.col);
    }
}

struct Token {
    data_type: String,
    value: String
}

// struct Language {
//     keywords: [String; 2]
// }

enum ReadEvent {
    IsWhiteSpace,
    IsOPChar,
    HasDot,
    IsID,
    IsNotNewline,
}