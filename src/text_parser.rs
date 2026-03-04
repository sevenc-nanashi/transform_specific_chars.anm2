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
            '\t' => {
                i += 1;
            }
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
