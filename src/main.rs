use base_page::end_file;
use elements::element;

mod base_page;
mod elements;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 4 {
        panic!("WRONG USAGE: qp <file> <output> <has_css>");
    }

    let input = std::fs::read_to_string(&args[1]).unwrap();
    let mut input = input.split(";").collect::<Vec<&str>>();
    if !input[0].starts_with("title: ") {
        panic!("ERROR: the page must have a title!")
    }

    let mut body = base_page::create_base_file(&input[0].replace("title: ", ""));
    input[0] = "";

    for line in input {
        let line = line.to_string().replace("\n", "");
        let (el, content) = line.split_once(": ").unwrap_or(("", ""));
        if el.is_empty() && content.is_empty() {
            continue;
        }
        let (opt, content) = match content.contains("]]") {
            true => content.split_once("]]").unwrap(),
            false => ("", content),
        };
        let opt = opt.replace("[[", "");

        body += &element(&el, &opt, content.to_string());
    }
    
    match &args[3] {
        x if x.to_lowercase() == "y" || x.to_lowercase() == "yes" => {
            let _ = std::fs::write(&args[2], body + &end_file(true));
        }

        _ => {
            let _ = std::fs::write(&args[2], body + &end_file(false));
        }
    }
    
}
