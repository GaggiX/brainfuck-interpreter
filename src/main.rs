use std::fs;
use std::env::args;
use std::io::{Result, stdin, Read};

enum Comm {
    Pointer(i32),           // <> (x)
    Plus(i32, u8),          // + (offset, x)
    Minus(i32, u8),         // - (offset, x)
    Dot(i32),               // . (offset)
    Comma(i32),             // , (offset)
    LP(usize),              // [ (pos)
    RP(usize),              // ] (pos)
    Clear(i32)              // setZero (pos)
}

fn interpreter(code: &Vec<Comm>) -> Result<()> {
    let mut cells: [u8; 65535] = [0; 65535];
    let mut index: i32 = 0;
    let mut ind_code: usize = 0;

    while ind_code < code.len() {
        match code[ind_code] {
            Comm::Pointer(i) => {
                index = index + i
            },
            Comm::Plus(off, i)  => {
                cells[(index + off) as usize] += i;
            },
            Comm::Minus(off, i) => {
                cells[(index + off) as usize] -= i;
            },
            Comm::Dot(off)      => print!("{}", cells[(index + off) as usize] as char),
            Comm::Comma(off)    => {
                cells[(index + off) as usize] = stdin()
                    .bytes()
                    .next()
                    .unwrap()?
                },
            Comm::LP(i) => {
                if cells[index as usize] == 0 {
                    ind_code = i; 
                }
            },
            Comm::RP(i) => {
                if cells[index as usize] != 0 {
                    ind_code = i;
                }
            },
            Comm::Clear(off) => cells[(index + off) as usize] = 0
        }
        ind_code += 1;
    }
    Ok(())
}

struct Bytecode {
    chars: Vec<u8>,
    index: usize,
    v:     Vec<Comm>
}

impl Bytecode {

    fn read_chars(&mut self, chr: char) -> i32 {
        let mut i = 0;
        while self.index != self.chars.len() && self.chars[self.index] as char == chr {
            i += 1;
            self.index += 1;
        }
        self.index -= 1;
        i
    }

    fn to_bytecode(&mut self, init: usize) {
        let mut offset = 0;

        while self.index < self.chars.len() {
            match self.chars[self.index] as char {
                '>' => {
                    let i = self.read_chars('>');
                    offset += i;
                },
                '<' => {
                    let i = self.read_chars('<');
                    offset -= i;
                },
                '+' => {
                    let i = self.read_chars('+');
                    self.v.push(Comm::Plus(offset, i as u8))
                },
                '-' => {
                    let i = self.read_chars('-');
                    self.v.push(Comm::Minus(offset, i as u8))
                },
                '.' => self.v.push(Comm::Dot(offset)),
                ',' => self.v.push(Comm::Comma(offset)),
                '[' => {
                    if self.chars[self.index + 1] as char == '-' && self.chars[self.index + 2] as char == ']' {
                        self.v.push(Comm::Clear(offset));
                        self.index += 2;
                    } else {
                        if offset != 0 {
                            self.v.push(Comm::Pointer(offset));
                            offset = 0;
                        }
                        //ugly workaround
                        let i = self.v.len();
                        self.v.push(Comm::Clear(0));
                        self.index += 1;
                        self.to_bytecode(self.v.len() - 1);
                        self.v[i] = Comm::LP(self.v.len() - 1);
                    }
                },
                ']' => {
                    if offset != 0 {
                        self.v.push(Comm::Pointer(offset));
                    }
                    self.v.push(Comm::RP(init));
                    return;
                },
                _ => ()
            }
            self.index += 1;
        }
    }
}


fn main() {
    let path = args()
        .skip(1)
        .next()
        .expect("should have one argument");

    let code = fs::read_to_string(path)
        .unwrap();

    let mut bytecode = Bytecode {
        chars: code.bytes().collect(),
        index: 0,
        v: Vec::new()
    };

    bytecode.to_bytecode(0);

    interpreter(&bytecode.v).unwrap();
}
