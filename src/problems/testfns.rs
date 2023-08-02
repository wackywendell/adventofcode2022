use anyhow::bail;

pub fn unindented(s: &str) -> anyhow::Result<String> {
    let mut input = String::new();

    let mut indent = None;
    for l in s.lines() {
        if indent.is_none() && l.trim().is_empty() {
            continue;
        }

        let l = l.trim_end_matches('\n');
        let indent = match indent {
            Some(i) => i,
            None => {
                let trimmed = l.trim_start_matches(' ');
                let i = l.len() - trimmed.len();

                indent = Some(i);
                i
            }
        };

        let l = unindent_line(l, indent)?;
        input.push_str(l);
        input.push('\n');
    }

    while let Some(end) = input.pop() {
        if end == '\n' {
            continue;
        }
        input.push(end);
        break;
    }

    Ok(input)
}

pub fn unindent_line(s: &str, indent: usize) -> anyhow::Result<&str> {
    let l = s.trim_end_matches('\n');

    let to_cut = indent.min(l.len());

    let spaces = &l[..to_cut];
    if !spaces.chars().all(|c| c == ' ') {
        bail!("Line has inconsistent indentation: {}", l);
    }

    Ok(&l[to_cut..])
}

pub fn unindent(s: &str, indent: usize) -> anyhow::Result<String> {
    let mut input = String::new();

    for l in s.lines() {
        if l.trim().is_empty() {
            if !input.is_empty() {
                input.push('\n');
            }
            continue;
        }

        let l = unindent_line(l, indent)?;
        input.push_str(l);
        input.push('\n');
    }

    while let Some(end) = input.pop() {
        if end == '\n' {
            continue;
        }
        input.push(end);
        break;
    }

    Ok(input)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_unindented() {
        let input = "
            Hello
            
            World
            Hello

            Again
        ";

        let expected = "Hello\n\nWorld\nHello\n\nAgain";

        assert_eq!(unindented(input).unwrap(), expected);
    }

    #[test]
    fn test_unindent() {
        let input = "
              Hello
            
            World
                Hello

            Again
        ";

        let expected = "  Hello\n\nWorld\n    Hello\n\nAgain";

        assert_eq!(unindent(input, 12).unwrap(), expected);
    }
}
