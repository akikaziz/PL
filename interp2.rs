use std::{
    env, error, fs,
    io::{self, Read, Write},
};
fn main() -> Result<(), Box<dyn error::Error>> {
    let prog = fs::read(env::args().nth(1).unwrap())?;

    // "b" is for bracket
    let mut bmap = vec![0; prog.len()]; // Map from a pc in the program to the jump location
    let mut bstack = vec![]; // Used to track nested brackets
    
    for (pc, &token) in prog.iter().enumerate() {
        match token as char {
            '[' => {
                bstack.push(pc);
            },
            ']' => {
                let start = bstack.pop().unwrap();
                bmap[start] = pc;
                bmap[pc] = start;
            },
            _ => {}
        }
    }

    let mut pc = 0;
    let mut cells = vec![0u8; 10000];
    let mut cc = 0;

    while pc < prog.len() {
        match prog[pc] as char {
            '<' => cc -= 1,
            '>' => cc += 1,
            '+' => cells[cc] = cells[cc].wrapping_add(1),
            '-' => cells[cc] = cells[cc].wrapping_sub(1),
            '[' if cells[cc] == 0 => pc = bmap[pc],
            ']' if cells[cc] != 0 => pc = bmap[pc],
            '.' => io::stdout().write_all(&cells[cc..cc + 1])?,
            ',' => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (),
        }
        pc += 1;
    }
    Ok(())
}
