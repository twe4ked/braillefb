//! A framebuffer that takes a `&[bool]` slice and returns 2x4 "dot" (pixel) [braille `char`s][1].
//!
//! [1]: https://en.wikipedia.org/wiki/Braille_Patterns
//!
//! # Example
//!
//! ```
//! # use braillefb::Framebuffer;
//! // ⣇⠽
//! let framebuffer = vec![
//!     true, false, true, true,
//!     true, false, false, true,
//!     true, false, true, true,
//!     true, true, false, false,
//! ];
//!
//! let f = Framebuffer::new(&framebuffer, 4, 4);
//!
//! // Get individual braille `char`s
//! assert_eq!(Some('⣇'), f.get(0));
//!
//! // As an iterator
//! let output: String = f.into_iter().collect();
//! assert_eq!("⣇⠽\n", &output);
//! ```

use std::fmt;

// https://en.wikipedia.org/wiki/Braille_Patterns
//
// 1 4
// 2 5
// 3 6
// 7 8
const BIT_OFFSETS: [(usize, usize); 8] = [
    (1, 3), // 8
    (0, 3), // 7
    (1, 2), // 6
    (1, 1), // 5
    (1, 0), // 4
    (0, 2), // 3
    (0, 1), // 2
    (0, 0), // 1
];
const CHAR_WIDTH: usize = 2;
const CHAR_HEIGHT: usize = 4;

/// A framebuffer that takes a `&[bool]` slice and returns 2x4 "dot" (pixel) [braille `char`s][1].
///
/// [1]: https://en.wikipedia.org/wiki/Braille_Patterns
///
/// # Example
///
/// ```
/// # use braillefb::Framebuffer;
/// // ⣇⠽
/// let framebuffer = vec![
///     true, false, true, true,
///     true, false, false, true,
///     true, false, true, true,
///     true, true, false, false,
/// ];
///
/// let f = Framebuffer::new(&framebuffer, 4, 4);
///
/// // Get individual braille `char`s
/// assert_eq!(Some('⣇'), f.get(0));
///
/// // As an iterator
/// let output: String = f.into_iter().collect();
/// assert_eq!("⣇⠽\n", &output);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Framebuffer<'a> {
    framebuffer: &'a [bool],
    width: usize,
    height: usize,
    x_chars_count: usize,
    y_chars_count: usize,
}

impl<'a> Framebuffer<'a> {
    /// Create a Framebuffer instance.
    ///
    /// # Panics
    ///
    /// Panics if length of supplied `framebuffer` slice is not equal to `width * height`.
    pub fn new(framebuffer: &'a [bool], width: usize, height: usize) -> Self {
        assert_eq!(
            framebuffer.len(),
            width * height,
            "supplied slice does not match width * height"
        );

        let x_chars_count = (round_up(width, CHAR_WIDTH) / CHAR_WIDTH) + 1; // + 1 for linebreaks
        let y_chars_count = round_up(height, CHAR_HEIGHT) / CHAR_HEIGHT;

        Self {
            framebuffer,
            width,
            height,
            x_chars_count,
            y_chars_count,
        }
    }

    /// Get the nth braille character in the framebuffer.
    ///
    /// # Example
    ///
    /// ```
    /// # use braillefb::Framebuffer;
    /// // ⣇⠽
    /// let framebuffer = vec![
    ///     true, false, true, true,
    ///     true, false, false, true,
    ///     true, false, true, true,
    ///     true, true, false, false,
    /// ];
    ///
    /// let f = Framebuffer::new(&framebuffer, 4, 4);
    ///
    /// assert_eq!(Some('⣇'), f.get(0));
    /// assert_eq!(Some('⠽'), f.get(1));
    /// assert_eq!(Some('\n'), f.get(2)); // Linebreaks are included
    /// assert_eq!(None, f.get(3));
    /// ```
    pub fn get(&self, index: usize) -> Option<char> {
        match self.offsets(index) {
            Offsets::Char(x_offset, y_offset) => Some(get_char(
                self.framebuffer,
                x_offset,
                y_offset,
                self.width,
                self.height,
            )),
            Offsets::Linebreak => Some('\n'),
            Offsets::End => None,
        }
    }

    /// Returns the number of braille `chars` across the image including a trailing linebreak.
    ///
    /// # Example
    ///
    /// ```
    /// # use braillefb::Framebuffer;
    /// # let framebuffer = vec![
    /// #     true, false, true, true,
    /// #     true, false, false, true,
    /// #     true, false, true, true,
    /// #     true, true, false, false,
    /// #     true, true, false, true,
    /// #     true, true, false, true,
    /// #     false, false, true, true,
    /// #     true, false, true, false,
    /// # ];
    /// // ⣇⠽
    /// // ⡛⡼
    /// let f = Framebuffer::new(&framebuffer, 4, 8);
    /// assert_eq!("⣇⠽\n⡛⡼\n", &f.to_string());
    /// assert_eq!(3, f.x_chars_count());
    /// ```
    pub fn x_chars_count(&self) -> usize {
        self.x_chars_count
    }

    /// Returns the number of braille `chars` down the image.
    ///
    /// # Example
    ///
    /// ```
    /// # use braillefb::Framebuffer;
    /// # let framebuffer = vec![
    /// #     true, false, true, true,
    /// #     true, false, false, true,
    /// #     true, false, true, true,
    /// #     true, true, false, false,
    /// #     true, true, false, true,
    /// #     true, true, false, true,
    /// #     false, false, true, true,
    /// #     true, false, true, false,
    /// # ];
    /// // ⣇⠽
    /// // ⡛⡼
    /// let f = Framebuffer::new(&framebuffer, 4, 8);
    /// assert_eq!("⣇⠽\n⡛⡼\n", &f.to_string());
    /// assert_eq!(2, f.y_chars_count());
    /// ```
    pub fn y_chars_count(&self) -> usize {
        self.y_chars_count
    }

    /// The number of `char`s (including linebreaks) that can be returned.
    pub fn len(&self) -> usize {
        self.y_chars_count * self.x_chars_count
    }

    pub fn is_empty(&self) -> bool {
        self.framebuffer.is_empty()
    }

    fn offsets(&self, index: usize) -> Offsets {
        if index > 0 && (index + 1) % self.x_chars_count == 0 {
            return Offsets::Linebreak;
        }

        let rows = index / self.x_chars_count;
        let y_offset = rows * CHAR_HEIGHT;

        if y_offset >= self.height {
            return Offsets::End;
        }

        let cols = index % self.x_chars_count;
        let x_offset = cols * CHAR_WIDTH;

        Offsets::Char(x_offset, y_offset)
    }
}

impl fmt::Display for Framebuffer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl<'a, 'f> IntoIterator for &'a Framebuffer<'f> {
    type Item = char;
    type IntoIter = Iter<'a, 'f>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self,
            index: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Offsets {
    Char(usize, usize),
    Linebreak,
    End,
}

/// An iterator over braille `char`s.
pub struct Iter<'a, 'i> {
    inner: &'a Framebuffer<'i>,
    index: usize,
}

impl<'a, 'i> Iterator for Iter<'a, 'i> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.offsets(self.index) {
            Offsets::Char(x_offset, y_offset) => {
                self.index += 1;
                Some(get_char(
                    self.inner.framebuffer,
                    x_offset,
                    y_offset,
                    self.inner.width,
                    self.inner.height,
                ))
            }
            Offsets::Linebreak => {
                self.index += 1;
                Some('\n')
            }
            Offsets::End => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.inner.len();
        (size, Some(size))
    }
}

fn round_up(input: usize, multiple: usize) -> usize {
    ((input + multiple - 1) / multiple) * multiple
}

// The x/y offsets are combined with the BIT_OFFSETS to create a u8 in the order that a
// UTF-8 braille character is represented
//
// 1 4
// 2 5
// 3 6
// 7 8
//
// 0b00000000
//   12345678
fn get_char(
    framebuffer: &[bool],
    x_offset: usize,
    y_offset: usize,
    width: usize,
    height: usize,
) -> char {
    let mut n: u8 = 0;
    for (x, y) in BIT_OFFSETS {
        n <<= 1;
        let xx = x_offset + x;
        let yy = y_offset + y;
        if xx >= width || yy >= height {
            continue;
        }
        n |= framebuffer[xx + yy * width] as u8;
    }
    to_braille(n)
}

fn to_braille(i: u8) -> char {
    char::from_u32(0x2800 + i as u32).expect("0x2800 + any u8 is a valid UTF-8 char")
}

#[cfg(test)]
mod tests {
    use super::{get_char, Framebuffer, Offsets};

    macro_rules! framebuffer {
        (#) => {true};
        (.) => {false};
        ($($c:tt)+) => {vec![
            $(framebuffer!($c)),+
        ]}
    }

    #[test]
    fn single_chars() {
        let framebuffer = framebuffer![
            # .
            # #
            . .
            . .
        ];
        let f = Framebuffer::new(&framebuffer, 2, 4);
        let mut f = f.into_iter();
        assert_eq!(Some('⠓'), f.next());
        assert_eq!(Some('\n'), f.next());
        assert_eq!(None, f.next());

        let framebuffer = framebuffer![
            # .
            # .
            # .
            # #
        ];
        let f = Framebuffer::new(&framebuffer, 2, 4);
        let mut f = f.into_iter();
        assert_eq!(Some('⣇'), f.next());
        assert_eq!(Some('\n'), f.next());
        assert_eq!(None, f.next());
    }

    #[test]
    fn multiple_chars() {
        // ⣇⠽
        // ⡛⡼
        let framebuffer = framebuffer![
            # . # #
            # . . #
            # . # #
            # # . .
            # # . #
            # # . #
            . . # #
            # . # .
        ];
        let f = Framebuffer::new(&framebuffer, 4, 8);
        let mut f = f.into_iter();

        assert_eq!(Some('⣇'), f.next());
        assert_eq!(Some('⠽'), f.next());
        assert_eq!(Some('\n'), f.next());
        assert_eq!(Some('⡛'), f.next());
        assert_eq!(Some('⡼'), f.next());
    }

    #[test]
    fn len() {
        // # #
        // # #
        // # #
        // # #
        // \n
        let framebuffer = vec![false; 2 * 4];
        let f = Framebuffer::new(&framebuffer, 2, 4);
        assert_eq!(2, f.len());

        // # # # #
        // # # # #
        // # # # #
        // # # # #
        // \n
        // # # # #
        // # # # #
        // # # # #
        // # # # #
        // \n
        let framebuffer = vec![false; 4 * 8];
        let f = Framebuffer::new(&framebuffer, 4, 8);
        assert_eq!(6, f.len());

        // # # # .
        // # # # .
        // # # # .
        // # # # .
        // \n
        // # # # .
        // . . . .
        // . . . .
        // . . . .
        // \n
        let framebuffer = vec![false; 3 * 5];
        let f = Framebuffer::new(&framebuffer, 3, 5);
        assert_eq!(6, f.len());
    }

    #[test]
    fn padding() {
        // ⠇⠅
        // ⠉⠁
        let framebuffer = framebuffer![
            # . #
            # . .
            # . #
            . . .
            # # #
        ];
        let f = Framebuffer::new(&framebuffer, 3, 5);
        let mut f = f.into_iter();

        assert_eq!(Some('⠇'), f.next());
        assert_eq!(Some('⠅'), f.next());
        assert_eq!(Some('\n'), f.next());

        assert_eq!(Some('⠉'), f.next());
        assert_eq!(Some('⠁'), f.next());
    }

    #[test]
    fn test_get_char() {
        // ⠇⠅
        // ⠉⠁
        let framebuffer = framebuffer![
            # . #
            # . .
            # . #
            . . .
            # # #
        ];

        assert_eq!('⠇', get_char(&framebuffer, 0, 0, 3, 5));
        assert_eq!('⠅', get_char(&framebuffer, 2, 0, 3, 5));
        assert_eq!('⠉', get_char(&framebuffer, 0, 4, 3, 5));
        assert_eq!('⠁', get_char(&framebuffer, 2, 4, 3, 5));
    }

    #[test]
    fn offsets() {
        // 0 # 1 #
        // # # # #
        // # # # #
        // # # # #
        // 2 \n
        // 3 # 4 #
        // # # # #
        // # # # #
        // # # # #
        // 5 \n
        // 6 # 7 #
        // # # # #
        // # # # #
        // # # # #
        // 8 \n
        let framebuffer = vec![false; 4 * 12];
        let f = Framebuffer::new(&framebuffer, 4, 12);

        // Row 1
        assert_eq!(Offsets::Char(0, 0), f.offsets(0));
        assert_eq!(Offsets::Char(2, 0), f.offsets(1));
        assert_eq!(Offsets::Linebreak, f.offsets(2));

        // Row 2
        assert_eq!(Offsets::Char(0, 4), f.offsets(3));
        assert_eq!(Offsets::Char(2, 4), f.offsets(4));
        assert_eq!(Offsets::Linebreak, f.offsets(5));

        // Row 3
        assert_eq!(Offsets::Char(0, 8), f.offsets(6));
        assert_eq!(Offsets::Char(2, 8), f.offsets(7));
        assert_eq!(Offsets::Linebreak, f.offsets(8));

        assert_eq!(Offsets::End, f.offsets(9));
    }

    #[test]
    fn offsets_with_padding() {
        // 0 # 1 #
        // # # # #
        // # # # #
        // # # # #
        // 2 \n
        // 3 # 4 #
        // # # # #
        // # # # #
        // # # # #
        // 5 \n
        // 6 # 7 #
        // # # # #
        // # # # #
        // # # # #
        // 8 \n

        // Width - 1 and height - 1 here to test offsets work when they need padding
        let framebuffer = vec![false; 3 * 11];
        let f = Framebuffer::new(&framebuffer, 3, 11);

        // Row 1
        assert_eq!(Offsets::Char(0, 0), f.offsets(0));
        assert_eq!(Offsets::Char(2, 0), f.offsets(1));
        assert_eq!(Offsets::Linebreak, f.offsets(2));

        // Row 2
        assert_eq!(Offsets::Char(0, 4), f.offsets(3));
        assert_eq!(Offsets::Char(2, 4), f.offsets(4));
        assert_eq!(Offsets::Linebreak, f.offsets(5));

        // Row 3
        assert_eq!(Offsets::Char(0, 8), f.offsets(6));
        assert_eq!(Offsets::Char(2, 8), f.offsets(7));
        assert_eq!(Offsets::Linebreak, f.offsets(8));

        assert_eq!(Offsets::End, f.offsets(9));
    }
}
