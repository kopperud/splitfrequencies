
pub fn tokenize(s: &str) -> Vec<&str> {
    let mut tokens: Vec<&str> = Vec::new();
    const SPECIAL_TOKENS: &[char] = &['(', ')', ',', ';'];
    let chars: Vec<char> = s.chars().collect();

    let mut start = 0;
    let mut end = 0;

    while end < chars.len(){
        let c = chars[end];
        if SPECIAL_TOKENS.contains(&c){
            if start != end {
                tokens.push(&s[start..end]);
            }
            if c != 0xA as char{
                tokens.push(&s[end..end+1]);
            }
            end += 1;
            start = end;
        }else{
            end += 1;
        }
    }

    if start != end{
        tokens.push(&s[start..end]);
    }

    tokens
}
