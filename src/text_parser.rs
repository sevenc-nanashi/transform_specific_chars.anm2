pub fn parse_text(text: &str) -> aviutl2::AnyResult<String> {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0usize;
    let mut out = Vec::new();

    while i < len {
        let c = chars[i];
        if !c.is_ascii() {
            out.push(c.to_string());
            i += 1;
            continue;
        }

        match c {
            '\\' => match chars.get(i + 1).copied() {
                Some('\\') => {
                    out.push("\\".to_string());
                    i += 2;
                }
                Some('n') => {
                    out.push("\n".to_string());
                    i += 2;
                }
                _ => {
                    out.push("\\".to_string());
                    i += 1;
                }
            },
            '<' => {
                let consumed = parse_control_sequence(&chars, i)?;
                if consumed == 0 {
                    out.push("<".to_string());
                    i += 1;
                } else {
                    i += consumed;
                }
            }
            _ => {
                out.push(c.to_string());
                i += 1;
            }
        }
    }

    Ok(out.join(""))
}

pub fn object_index_to_string_index(text: &str, index: usize) -> Option<usize> {
    // NOTE: 改行とタブはオブジェクトとして描画されないので、それを考慮して文字インデックスを計算する必要がある
    let chars: Vec<char> = text.chars().collect();
    let mut remaining = index;
    for (i, &c) in chars.iter().enumerate() {
        if c == '\n' || c == '\t' {
            // 改行とタブはオブジェクトとして描画されないので、文字インデックスを減らさない
        } else {
            if remaining == 0 {
                return Some(i);
            }
            remaining -= 1;
        }
    }
    None
}

fn parse_control_sequence(chars: &[char], i: usize) -> aviutl2::AnyResult<usize> {
    let Some(next) = chars.get(i + 1).copied() else {
        return Ok(0);
    };
    let rest: String = chars[i..].iter().collect();

    let consumed = match next {
        '#' | '@' => consume_color_or_font_tag(&rest),
        's' => consume_s_tag(&rest),
        'r' => consume_r_tag(&rest),
        'w' => consume_w_tag(&rest),
        'c' => {
            if lazy_regex::regex_is_match!(r"^<c[0-9.]+>", &rest) {
                aviutl2::anyhow::bail!("テキストに表示クリア記法が含まれています");
            }
            0
        }
        'p' => consume_p_tag(&rest),
        '?' => {
            if lazy_regex::regex_is_match!(r"^<\?[\s\S]*\?>", &rest) {
                aviutl2::anyhow::bail!("テキストにスクリプト記法が含まれています");
            }
            0
        }
        _ => 0,
    };

    Ok(consumed)
}

fn consume_color_or_font_tag(input: &str) -> usize {
    lazy_regex::regex_find!(r"^<[#@][^>]*>", input)
        .map(|matched| matched.chars().count())
        .unwrap_or(0)
}

fn consume_s_tag(input: &str) -> usize {
    lazy_regex::regex_find!(r"^<s[0-9.]*(?:,[^,]*(?:,[BIS]*(?:,[0-9.]+)?)?)?>", input)
        .map(|matched| matched.chars().count())
        .unwrap_or(0)
}

fn consume_r_tag(input: &str) -> usize {
    lazy_regex::regex_find!(r"^<r[0-9.]*>", input)
        .map(|matched| matched.chars().count())
        .unwrap_or(0)
}

fn consume_w_tag(input: &str) -> usize {
    lazy_regex::regex_find!(r"^<w\*?[0-9.]+>", input)
        .map(|matched| matched.chars().count())
        .unwrap_or(0)
}

fn consume_p_tag(input: &str) -> usize {
    lazy_regex::regex_find!(r"^<p[+-]?[0-9.]+,[+-]?[0-9.]+>", input)
        .map(|matched| matched.chars().count())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_text() {
        let input = r"Hello\nWorld<#FF0000><s1,2,BI><r0.5><w*1.5><p10,-5>Red";
        let expected = "Hello\nWorldRed";
        let result = parse_text(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_object_index_to_string_index() {
        let text = "A\nB\tC";
        assert_eq!(object_index_to_string_index(text, 0), Some(0)); // 'A'
        assert_eq!(object_index_to_string_index(text, 1), Some(2)); // 'B'
        assert_eq!(object_index_to_string_index(text, 2), Some(4)); // 'C'
        assert_eq!(object_index_to_string_index(text, 3), None); // Out of bounds
    }
}
