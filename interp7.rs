use std::{
    env, error, fs,
    io::{self, Read, Write},
};
enum Ops {
    Left(usize),
    Right(usize),
    Add(u8),
    Sub(u8),
    LBrack(usize),
    RBrack(usize),
    Zero,
    Output,
    Input,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prog = vec![];
    let bytes = fs::read(env::args().nth(1).unwrap())?;
    let mut i = 0;

    // Compile
    while i < bytes.len() {
        match bytes[i] as char {
            '>' => {
                let count = bytes[i..].iter().take_while(|&&x| x == b'>').count();
                prog.push(Ops::Right(count));
                i += count as usize - 1;
            }
            '<' => {
                let count = bytes[i..].iter().take_while(|&&x| x == b'<').count();
                prog.push(Ops::Left(count));
                i += count as usize - 1;
            }
            '-' => {
                let count = bytes[i..].iter().take_while(|&&x| x == b'-').count();
                prog.push(Ops::Sub(count as u8));
                i += count as usize - 1;
            }
            '+' => {
                let count = bytes[i..].iter().take_while(|&&x| x == b'+').count() as u8;
                prog.push(Ops::Add(count));
                i += count as usize - 1;
            }
            '[' => prog.push(Ops::LBrack(usize::max_value())),
            ']' => prog.push(Ops::RBrack(usize::max_value())),
            '.' => prog.push(Ops::Output),
            ',' => prog.push(Ops::Input),
            _ => (),
        }
        i += 1;
    }

    // Optimize
    /* Iterate through the program, in search of our "Zero" optimization */
    i = 0;
    while i < prog.len() - 2 {
        if let (Ops::LBrack(_), Ops::Sub(1), Ops::RBrack(_)) = (&prog[i], &prog[i+1], &prog[i+2]) {
            prog[i] = Ops::Zero;
            prog.drain(i+1..=i+2);
        }
        i += 1;
    }

    let mut bstack = vec![];
    let mut i = 0;
    
    while i < prog.len() {
        match &mut prog[i] {
            Ops::LBrack(_) => {
                bstack.push(i);
            }
            Ops::RBrack(_) => {
                if let Some(start) = bstack.pop() {
                    match &mut prog[i] {
                        Ops::RBrack(jump_pos) => {
                            *jump_pos = start;
                            match &mut prog[start] {
                                Ops::LBrack(jump_pos) => {
                                    *jump_pos = i;
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        i += 1;
    }

    // Interpret / Evaluate
    let mut cells = vec![0u8; 10000];
    let mut cc = 0usize;
    let mut pc = 0;
    while pc < prog.len() {
        match &prog[pc] {
            Ops::Zero => cells[cc] = 0,
            Ops::Right(count) => cc = cc.wrapping_add(*count),
            Ops::Left(count) => cc = cc.wrapping_sub(*count),
            Ops::Add(count) => {
                cells[cc] = cells[cc].wrapping_add(*count);
            }
            Ops::Sub(count) => cells[cc] = cells[cc].wrapping_sub(*count as u8),
            Ops::LBrack(jump_pos) => {
                if cells[cc] == 0 {
                    pc = *jump_pos;
                }
            }
            Ops::RBrack(jump_pos) => {
                if cells[cc] != 0 {
                    pc = *jump_pos;
                }
            }
            Ops::Output => io::stdout().write_all(&cells[cc..cc + 1])?,
            Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (),
        }
        pc += 1;
    }
    Ok(())
}
