pub type FI = FileInfo;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FileInfo {
    pub line: usize,
    pub column: usize,
    pub length: usize,
    pub offset: usize,
}

impl FileInfo {
    pub fn new(line: usize, column: usize, length: usize, offset: usize) -> FileInfo {
        FileInfo {
            line,
            column,
            length,
            offset,
        }
    }
    pub fn zero() -> FileInfo {
        FileInfo {
            line: 0,
            column: 0,
            length: 0,
            offset: 0,
        }
    }

    pub fn col_inc(&mut self) {
        self.column += 1;
        self.length += 1;
        self.offset += 1;
    }
    pub fn line_inc(&mut self) {
        self.line += 1;
        self.length += 1;
        self.offset += 1;
        // reset column
        self.column = 1;
    }

    pub fn len_diff(&self, start: &FileInfo) -> FileInfo {
        FileInfo::new(start.line, start.column, self.length - start.length, start.offset)
    }

    pub fn merge(&self, end: &FileInfo) -> FileInfo {
        FileInfo::new(self.line, self.column, end.offset + end.length - self.offset, self.offset)
    }
}
