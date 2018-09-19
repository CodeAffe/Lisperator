extern crate regex;

mod InputStream;
mod TokenStream;

fn main() {
    let mut input = InputStream::InputStream {
        pos: 0,
        line: 1,
        col: 0,
        input: "t\nest".to_string().chars().collect()
    };
    
    let mut token = TokenStream::TokenStream {
        pos: 0,
        line: 1,
        col: 0,
        input: input,
        keywords: ["if".to_string(), "then".to_string(), "else".to_string()]
    };
    
    let retrun = token.is_digit('1');
    
    println!("Return: {}", retrun);
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