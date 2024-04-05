pub struct Text {
    pub text: String,
}
pub struct Tag {
    pub tag: String,
}

pub enum Token {
    Text(Text),
    Tag(Tag),
}

pub fn lex(body: String) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut buffer = "".to_string();
    let mut in_tag = false;
    for c in body.chars() {
        if c == '<' {
            in_tag = true;
            if !buffer.is_empty() {
                output.push(Token::Text(Text {
                    text: buffer.clone(),
                }));
            }
            buffer = "".to_string();
        } else if c == '>' {
            in_tag = false;
            output.push(Token::Tag(Tag {
                tag: buffer.clone(),
            }));
            buffer = "".to_string();
        } else {
            buffer = format!("{}{}", buffer, c);
        }
    }
    if !in_tag && !buffer.is_empty() {
        output.push(Token::Text(Text { text: buffer }))
    }
    output
}
