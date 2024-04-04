pub fn lex(body: String) -> String {
    let mut in_tag = false;
    let mut text = "".to_string();
    for c in body.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            text.push(c);
        }
    }
    text
}
