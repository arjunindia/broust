use ::std::env;

mod networking;

fn show_body(body: String) {
    let mut in_tag = false;
    for c in body.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            print!("{}", c)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("Not enough arguments! add a `-- {{url}}` at the end of the CLI");
        return;
    }
    let url = networking::url::URL::new(&args[1]);
    let response = url.request();
    show_body(response);
    println!("\n\nConnection Scheme: {}", url.scheme);
    println!("Connection Host: {}", url.host);
    println!("Connection Path: {}", url.path);
}
