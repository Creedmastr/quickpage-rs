mod base_page;
mod elements;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if !args.len() < 3 {
        panic!("WRONG USAGE: qp <file> <title>");
    }
    
    let input = std::fs::read_to_string(&args[1]).unwrap();

    let mut base_file = &String::new();

    let input = input.chars();

    let mut buffer = String::new();
    let mut char_count: u32 = 0;
    for char in input {
        let char = char.to_string();

        match char == ";" {
            true => {
                match &buffer {
                    x if x == "<" || x == ">" => {
                        buffer = String::new();
                        continue;
                    }

                    x if x == "title" => {
                        base_page::create_base_file(x);
                    }

                    _ => {
                        panic!("Unknown expression at char: {0}, {1}", char_count, buffer);
                    }
                }

                buffer = String::new();
            }

            false => {
                buffer += &char;
            }
        }

        char_count += 1;
    }
}
