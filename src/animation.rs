use super::NUM_DIGITS;

pub struct ScrollingText<'a> {
    content: &'a str,
    idx: usize,
    reverse: bool,
}

impl<'a> ScrollingText<'a> {
    pub fn new(string: &'a str) -> ScrollingText<'a> {
        ScrollingText {
            content: string,
            idx: 0,
            reverse: false,
        }
    }

    pub fn destroy(self) {
        return;
    }

    /// Returns the current string to display and advances it afterwards
    pub fn display(&mut self) -> &str {
        if self.content.len() <= NUM_DIGITS {
            return self.content; // If content fits on display no scrolling is necessary
        }

        let current = &self.content[self.idx..self.idx + NUM_DIGITS];

        if self.idx + NUM_DIGITS >= self.content.len() {
            self.reverse = true;
        }

        if self.idx <= 0 {
            self.reverse = false;
        }

        if !self.reverse {
            self.idx += 1;
        } else {
            self.idx -= 1;
        }

        current
    }
}
