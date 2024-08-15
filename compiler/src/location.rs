use core::fmt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Location {
    row: usize,
    column: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {} column {}", self.row, self.column)
    }
}

impl Location {
    pub fn visualize<'a>(
        &self,
        line: &'a str,
    ) -> impl fmt::Display + 'a {
        struct Visualize<'a> {
            loc: Location,
            line: &'a str,
        }
        impl fmt::Display for Visualize<'_> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{}\n{arrow:>pad$}",
                    self.line,
                    pad = self.loc.column,
                    arrow = "^",
                )
            }
        }
        Visualize {
            loc: *self,
            line,
        }
    }
}

impl Location {
    pub fn new(row: usize, column: usize) -> Self {
        Location { row, column }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn reset(&mut self) {
        self.row = 1;
        self.column = 1;
    }

    pub fn go_right(&mut self) {
        self.column += 1;
    }

    pub fn go_left(&mut self) {
        self.column -= 1;
    }

    pub fn newline(&mut self) {
        self.row += 1;
        self.column = 1;
    }
}
