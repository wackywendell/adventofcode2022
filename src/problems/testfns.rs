use anyhow::bail;

pub fn unindent(s: &str) -> anyhow::Result<String> {
    let mut input = String::new();

    let mut indent = None;
    for (n, l) in s.lines().enumerate() {
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

        let to_cut = indent.min(l.len());

        let spaces = &l[..to_cut];
        if !spaces.chars().all(|c| c == ' ') {
            bail!("Line {} has inconsistent indentation: {}", n + 1, l);
        }

        let l = &l[to_cut..];
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
    fn test_unindent() {
        let input = "
            Hello
            
            World
            Hello

            Again
        ";

        let expected = "Hello\n\nWorld\nHello\n\nAgain";

        assert_eq!(unindent(input).unwrap(), expected);
    }
}
