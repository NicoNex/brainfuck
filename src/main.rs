use std::io::{self, Read, Write};

fn getchar() -> u8 {
    return io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .unwrap_or(0);
}

fn run(input: String) {
    const TAPE_SIZE: usize = 65536;
    let mut ptr = 0;
    let mut tape: Vec<u8> = vec![0; TAPE_SIZE];
    let mut cptr = 0;
    let code: Vec<char> = input.chars().collect();

    while cptr < code.len() {
        match code[cptr] {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => tape[ptr] += 1,
            '-' => tape[ptr] -= 1,
            '.' => print!("{}", tape[ptr] as char),
            ',' => tape[ptr] = getchar(),
            '[' => {
                if tape[ptr] == 0 {
                    let mut loops = 1;
                    while loops > 0 {
                        cptr += 1;
                        if cptr >= code.len() {
                            break;
                        }

                        loops += match code[cptr] {
                            '[' => 1,
                            ']' => -1,
                            _ => 0,
                        };
                    }
                }
            }
            ']' => {
                if tape[ptr] != 0 {
                    let mut loops = 1;
                    while loops > 0 && cptr > 0 {
                        cptr -= 1;

                        loops += match code[cptr] {
                            '[' => -1,
                            ']' => 1,
                            _ => 0,
                        }
                    }
                }
            }
            _ => {}
        }
        cptr += 1;
    }
}

fn read_line(prompt: &str) -> String {
    let mut line = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut line)
        .expect("failed to read line");
    line.trim().to_string()
}

fn main() {
    loop {
        run(read_line("% "));
    }
}
