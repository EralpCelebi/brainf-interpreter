
#[derive(Copy,Clone,Debug,PartialEq)]
enum Tokens {
	Left,
	Right,
	Add,
	Sub,
	Get,
	Put,
	Start,
	End,
}
		
struct Interpreter {
	input: String,
	pointer: usize,
	memory: [u8;30000],
	tokens: Vec<Tokens>
}

impl Interpreter {
	pub fn new(input: String) -> Self {
		Self {
			input,
			pointer: 0,
			memory: [0;30000],
			tokens: Vec::new()
		}
	}

	pub fn lex(&mut self) {
		for chr in self.input.chars() {
			match chr {
				'<' => self.tokens.push(Tokens::Left),
				'>' => self.tokens.push(Tokens::Right),
				'+' => self.tokens.push(Tokens::Add),
				'-' => self.tokens.push(Tokens::Sub),
				',' => self.tokens.push(Tokens::Get),
				'.' => self.tokens.push(Tokens::Put),
				'[' => self.tokens.push(Tokens::Start),
				']' => self.tokens.push(Tokens::End),
				_ => continue,
			}
		}
	}

	pub fn parse(&mut self,counter: usize) {
		let mut buffer = counter;
		while buffer < self.tokens.len() {
			match self.tokens[buffer] {
				Tokens::Left => {
					if self.pointer > 0 {
						self.pointer -= 1;
					}		
				},
				Tokens::Right => {
					if self.pointer < 30000 {
						self.pointer += 1;
					}
				},
				Tokens::Add => {
					if self.memory[self.pointer] < 255 {
						self.memory[self.pointer] += 1;
					}
				},
				Tokens::Sub => {
					if self.memory[self.pointer] > 0 {
						self.memory[self.pointer] -= 1;
					}
				},
				Tokens::Get => {
					use std::io::Read;
					
					let input: Option<u8> = std::io::stdin()
					    .bytes() 
					    .next()
					    .and_then(|result| result.ok())
					    .map(|byte| byte as u8);

					self.memory[self.pointer] = input.unwrap();
				},
				Tokens::Put => {
					use std::io::Write;
					let out = self.memory[self.pointer] as char;
					print!("{}",out);
					std::io::stdout().flush();
				},
				Tokens::Start => {
					let pointer_buffer = self.pointer;
					if self.memory[pointer_buffer] != 0 {
						while self.memory[pointer_buffer] != 0 {
							self.parse(buffer+1);
						}

						while self.tokens[buffer] != Tokens::End {
							//println!("{:#?}",self.tokens[buffer]);
							buffer += 1;
						}
					} else {
						while self.tokens[buffer] != Tokens::End {
							//println!("{:#?}",self.tokens[buffer]);
							buffer += 1;
						}
					}
				}
				Tokens::End => {
					return;
				}
			}
			buffer += 1;
		}
	}

	pub fn exec(&mut self) {
		self.lex();
		self.parse(0);
	}
}

fn main() {
	use std::io::{stdin,stdout,Write,Read};

	while true {
		print!("> ");
		stdout().flush();
		
		let mut input = String::new();
		
		stdin().read_line(&mut input)
			.expect("Failed while trying to read input.");
		
		let mut intr = Interpreter::new(input);
		intr.exec();
		println!();
	}
}