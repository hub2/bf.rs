use std::io;
use std::io::Read;
use std::process;
const MEMORY_SIZE: usize = 30000;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    buffer.pop();
    run(&buffer);
    Ok(())
}

fn run(code: &String) {
    let mut codeptr: usize = 0;
    let mut memoryptr: usize = 0;
    let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
    let mut loop_stack: Vec<usize> = Vec::new();
    loop {
        if codeptr >= code.len() {
            break;
        }
        let instr = code.chars().nth(codeptr).unwrap();
        //println!("{}", instr);
        match instr {
            '+' => memory[memoryptr] = memory[memoryptr].wrapping_add(1),
            '-' => memory[memoryptr] = memory[memoryptr].wrapping_sub(1),
            '[' => {
                let mut balance = 1;
                if memory[memoryptr] == 0 {
                    for i in codeptr + 1..MEMORY_SIZE {
                        let current_char = code.chars().nth(i).unwrap();

                        if current_char as char == '[' {
                            balance += 1
                        };
                        if current_char as char == ']' {
                            balance -= 1;
                            if balance == 0 {
                                codeptr = i;
                                break;
                            }
                        }
                    }
                } else {
                    let top = loop_stack.last();
                    if !top.is_none() {
                        if *top.unwrap() != codeptr {
                            loop_stack.push(codeptr)
                        }
                    } else {
                        loop_stack.push(codeptr)
                    }
                }
            }
            ']' => {
                let top = loop_stack.pop();
                if !top.is_none() {
                    codeptr = top.unwrap() - 1;
                } else {
                    println!("panic: Couldn't find loop start!");
                    process::exit(1);
                }
            }
            '>' => memoryptr += 1,
            '<' => memoryptr -= 1,
            '.' => print!("{}", memory[memoryptr] as char),
            ',' => {
                memory[memoryptr] = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u8)
                    .unwrap();
            }
            _default => {
                //println!("panic: Incorrect character!");
                //process::exit(1);
            }
        }
        codeptr += 1;
    }
}
