pub type FI = FileInfo;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FileInfo {
    pub length: usize,
    pub offset: usize,
}

impl FileInfo {
    pub fn new(length: usize, offset: usize) -> FileInfo {
        FileInfo { length, offset }
    }
    pub fn zero() -> FileInfo {
        FileInfo { length: 0, offset: 0 }
    }

    pub fn inc(&mut self) {
        self.length += 1;
        self.offset += 1;
    }

    pub fn len_diff(&self, start: &FileInfo) -> FileInfo {
        FileInfo::new(self.length - start.length, start.offset)
    }

    pub fn merge(&self, end: &FileInfo) -> FileInfo {
        FileInfo::new(end.offset + end.length - self.offset, self.offset)
    }
}


pub fn underline_error(input: &str, fi: &FileInfo) -> String {
    underline(input, fi, "\x1b[31m") // underlined in red
}

pub fn underline(input: &str, fi: &FileInfo, color: &str) -> String {
        let mut offset = 0;
        let mut col = 1;
        let mut l_num = 1;

        let mut prev_non_ws_line_offset = 0;
        let mut prev_non_ws_line_num = 1;

        
        let mut ti = input.chars().peekable();
        while offset < fi.offset {
            let c = ti.next().unwrap();
            match c {
                '\n' => {
                    l_num += 1;
                    col = 1;
                }
                ' ' | '\t' => col += 1,
                _ => {
                    prev_non_ws_line_offset = offset - (col - 1);
                    prev_non_ws_line_num = l_num;
                    col += 1;
                }
            }
            offset += 1;
        }

        let mut ti = input.chars().skip(prev_non_ws_line_offset).peekable();
        let mut line_number = prev_non_ws_line_num;
        let mut out_str = format!("{:3}: ", line_number);
        let mut offset = prev_non_ws_line_offset;
        // create 
        while offset < fi.offset {
            let c = ti.next().unwrap();
            match c {
                '\n' => {
                    line_number += 1;
                    out_str.push_str(&format!("\n{:3}: ", line_number));
                }
                _ => out_str.push(c),
            }
            offset += 1;
        }
        // add til the end of the line
        while let Some(c) = ti.next() {
            match c {
                '\n' => {
                    out_str.push_str("\n");
                    break;
                }
                _ => out_str.push(c),
            }
        }
        if !out_str.ends_with('\n') {
            out_str.push_str("\n");
        }
        out_str.push_str(color); // color escape sequence
        out_str.push_str(&format!("     {:->1$}", "^", col));
        if color.len() > 0 {
            out_str.push_str("\x1b[0m"); // reset
        }
        out_str

}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_underline() {
        let input = r#"0123456789"#;
        assert_eq!(underline(input, &FI::new(1, 0), ""), r#"
  1: 0123456789
     ^"#[1..]);
        assert_eq!(underline(input, &FI::new(1, 1), ""), r#"
  1: 0123456789
     -^"#[1..]);
        assert_eq!(underline(input, &FI::new(1, 8), ""), r#"
  1: 0123456789
     --------^"#[1..]);
    }

    #[test]
    fn test_underline_with_previous_non_white_on_other_line() {
        let input = r#"01234

789"#;
        assert_eq!(underline(input, &FI::new(1, 7), ""), r#"
  1: 01234
  2: 
  3: 789
     ^"#[1..]);
        assert_eq!(underline(input, &FI::new(1, 8), ""), r#"
  3: 789
     -^"#[1..]);
    }
    
}
