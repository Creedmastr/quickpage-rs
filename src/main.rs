use base_page::end_file;

mod base_page;
mod elements;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if !args.len() < 3 {
        panic!("WRONG USAGE: qp <file> <title>");
    }

    let input = std::fs::read_to_string(&args[1]).unwrap();
    let input = input.split(";").collect::<Vec<&str>>();
    if !input[0].starts_with("title: ") {
        panic!("ERROR: the page must have a title!")
    }

    let mut body = base_page::create_base_file(&input[0].replace("title: ", ""));

    let mut line_c: u32 = 0;
    for line in input {
        let line = line.to_string().replace("\n", "");
        match &line {
            x if x.starts_with("p: ") => {
                body += &format!("<p>{}</p>", line.replace("p: ", ""));
            }

            _ => {
                eprintln!(
                    "WARNING: Unknown expression at lines: {0}, '{1}', skipped",
                    line_c, line
                );
            }
        }

        line_c += 1;
    }

    let _ = std::fs::write(&args[2], body + &end_file());
}
