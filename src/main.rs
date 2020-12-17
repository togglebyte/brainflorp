use std::io::{Write, stdout};

fn run(source: &str) {
    let mut program = [0u8;30000];
    let mut ip = 0;
    let mut dp = 0;

    let mut stdout = stdout();
    let source = source.as_bytes();
    
    loop {
        let inst = source[ip];
        match inst {
            b'>' => dp += 1,
            b'<' => dp -= 1,
            b'+' => program[dp] += 1,
            b'-' => program[dp] -= 1,
            b'.' => { write!(stdout, "{}", program[dp] as char); }
            b',' => unimplemented!(),
            b'[' => if program[dp] == 0 { // jump to command AFTER ]
                let mut cntr = 0;
                loop {
                    ip += 1;
                    if source[ip] == b'[' { cntr += 1; }
                    if source[ip] == b']' {
                        if cntr == 0 {
                            break
                        }
                        cntr -= 1;
                    }
                }

            }
            b']' => if program[dp] != 0 { // Jump to the command AFTER [
                let mut cntr = 0;
                loop {
                    ip -= 1;
                    if source[ip] == b']' {
                        cntr += 1;
                    }
                    if source[ip] == b'[' {
                        if cntr == 0 {
                            break
                        }
                        cntr -= 1;
                    }
                }
            }
            _ => {}
        }

        ip += 1;

        if source.len() == ip {
            break
        }
    }

    stdout.flush();
}

fn main() {
    let source = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    run(source);
}
