use core::marker::PhantomData;

use super::NUM_DIGITS;

pub mod mode {
    pub trait Mode {}
    pub struct Cycle;
    impl Mode for Cycle {}
    pub struct LeftRight;
    impl Mode for LeftRight {}
}
use mode::*;

/// Text that has a window scrolling over it
///
/// [ScrollingText::get_next()] returns an iterator which is a moving window on the text. It yields 12 characters and is moved by one character every time the function is called.
pub struct ScrollingText<'a, MODE> {
    content: &'a str,
    idx: usize,
    reverse: bool,
    _mode: PhantomData<MODE>,
}

impl<'a, M: Mode> ScrollingText<'a, M> {
    pub fn new(string: &'a str, _mode: M) -> ScrollingText<'a, M> {
        ScrollingText {
            content: string,
            idx: 0,
            reverse: false,
            _mode: PhantomData,
        }
    }

    pub fn destroy(self) {
        return;
    }
}

impl ScrollingText<'_, Cycle> {
    /// Get cycling text
    ///
    /// The window wraps to the start of the text if the end is reached.
    pub fn get_next(&mut self) -> core::iter::Skip<core::iter::Cycle<core::str::Chars>> {
        let disp_iter = self.content.chars().cycle().skip(self.idx);
        self.idx = self.idx + 1 % NUM_DIGITS;

        disp_iter
    }
}
impl ScrollingText<'_, LeftRight> {
    /// Get a scrolling window which changes direction when reaching the text bounds
    pub fn get_next(&mut self) -> core::str::Chars {
        if self.content.len() <= NUM_DIGITS {
            return self.content.chars(); // If content fits on display no scrolling is necessary
        }

        let current = self.content[self.idx..self.idx + NUM_DIGITS].chars();

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
