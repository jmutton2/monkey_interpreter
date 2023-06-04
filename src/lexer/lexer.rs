pub struct Lexer {
    input: String,
    position: u32,
    read_position: u32,
    ch: char,
}

pub fn new(input: String) -> Lexer {
    let l = &Lexer{ input: input, position: 0, read_position: 0,  ch: 'a' };
    return *l;
}
