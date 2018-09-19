pub struct InputStream{
    pub pos: i32,
    pub line: i32,
    pub col: i32,
    pub input: Vec<char>
}

impl InputStream {
    /**
    * Returns the next value and discards it from stream
    */
    pub fn next(&mut self) -> char {
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
    pub fn peek(&self) -> char {
        self.input[self.pos as usize]
    }
    
    // Returns true if the stream is at the end
    pub fn eof(&self) -> bool {
        self.peek().to_string() == ""
    }
    
    // Throws an error
    pub fn croak(&self, msg: String) {
        panic!("{} ({}:{})", msg, self.line, self.col);
    }
}
