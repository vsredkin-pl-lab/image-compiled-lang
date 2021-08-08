#[derive(Clone, Debug)]
pub enum Token {
    LParen,
    RParen,

    Plus, Minus, Star, Slash,

    Name(usize),
    Print,
    Num(i64),
    Assign(usize),
    Define(usize)
}

pub enum LexicalError {

}

pub fn tokenize(s: & [u8]) -> Vec<(usize, Token, usize)> {
    use Token::*;

    let mut res :Vec<(usize, Token, usize)> = vec![];
    let mut iterator = s.iter().enumerate().peekable();
    while let (Some((idx, r)), Some(_), Some(_) )
    = (iterator.next(), iterator.next(), iterator.next()) {
        let start = idx;
        let token = match *r { //tokenize according to token list in icl-transpiler readme
            0 => LParen,
            1 => RParen,
            2 => Plus,
            3 => Minus,
            4 => Star,
            5 => Slash,
            6 => { //variable.
                //read next pixel
                let (r,g,b)
                    = (iterator.next(), iterator.next(), iterator.next());
                let (r, g, b)
                    = (*r.unwrap().1, *g.unwrap().1, *b.unwrap().1);

                let id:usize = (r as usize)<<16 | ((g as usize) << 8) | (b as usize);
                Name(id)
            },
            7 => Print,
            8 => {
                let mut number = 0u64;

                for _ in 0..4 {
                    let (r,g,b)
                        = (iterator.next(), iterator.next(), iterator.next());
                    let (r, g, _b)
                        = (*r.unwrap().1, *g.unwrap().1, *b.unwrap().1);
                    let sub = (r as u64) <<8 | (g as u64); //blue is ignored
                    number = number<<16 | sub;
                }
                Num(number as i64)
            },

            9 => {
                let (r,g,b)
                    = (iterator.next(), iterator.next(), iterator.next());
                let (r, g, b)
                    = (*r.unwrap().1, *g.unwrap().1, *b.unwrap().1);
                let id:usize = (r as usize)<<16 | ((g as usize) << 8) | (b as usize);
                Assign(id)
            },

            10 => {
                let (r,g,b)
                    = (iterator.next(), iterator.next(), iterator.next());
                let (r, g, b)
                    = (*r.unwrap().1, *g.unwrap().1, *b.unwrap().1);
                let id:usize = (r as usize)<<16 | ((g as usize) << 8) | (b as usize);
                Define(id)
            },
            _ => {continue;}, // ignore other pixels / blank space

        };
        let end = iterator.peek().unwrap_or(&(s.len(), &0u8)).0;
        res.push((start/3, token, end/3));
        // divide by 3 - single pixel is encoded with 3 bytes

    }
    res
}