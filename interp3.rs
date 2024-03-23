use std::{
    env, error, fs,
    io::{self, Read, Write},
};
enum Ops {
    Left,
    Right,
    Add,
    Sub,
    LBrack,
    RBrack,
    Output,
    Input,
}
fn main() -> Result<(), Box<dyn error::Error>> {
    /* Notice: prog is now a vec of OpCodes, not a string */
    let mut prog = vec![];

    /* First parse the program into a sequence of opcodes */
    for b in fs::read(env::args().nth(1).unwrap())? {
        match b as char {
            '<' => prog.push(Ops::Left),
            '>' => prog.push(Ops::Right),
            '+' => prog.push(Ops::Add),
            '-' => prog.push(Ops::Sub),
            '[' => prog.push(Ops::LBrack),
            ']' => prog.push(Ops::RBrack),
            '.' => prog.push(Ops::Output),
            ',' => prog.push(Ops::Input),
            _ => (),
        }
    }

    let mut pc = 0;
    let mut bmap = vec![0; prog.len()];
    let mut bstack = vec![];
    
    for (pc, op) in prog.iter().enumerate() {
        if let Ops::LBrack = op {
            bstack.push(pc);
        } else if let Ops::RBrack = op {
            let start = bstack.pop().unwrap();
            bmap[start] = pc;
            bmap[pc] = start;
        }
    }

    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    while pc < prog.len() {
        match prog[pc] {
            Ops::Left => cc -= 1,
            Ops::Right => cc += 1,
            Ops::Add => cells[cc] = cells[cc].wrapping_add(1),
            Ops::Sub => cells[cc] = cells[cc].wrapping_sub(1),
            Ops::LBrack if cells[cc] == 0 => pc = bmap[pc],
            Ops::RBrack if cells[cc] != 0 => pc = bmap[pc],
            Ops::Output => io::stdout().write_all(&cells[cc..cc + 1])?,
            Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (),
        }
        pc += 1;
    }
    Ok(())
}
