/// HCS-12SS59T Font Table
#[repr(u8)]
pub enum FontTable {
    /// Character `@`
    ChatAt = 0x10,
    /// Character `A`
    CharA = 0x11,
    /// Character `B`
    CharB = 0x12,
    /// Character `C`
    CharC = 0x13,
    /// Character `D`
    CharD = 0x14,
    /// Character `E`
    CharE = 0x15,
    /// Character `F`
    CharF = 0x16,
    /// Character `G`
    CharG = 0x17,
    /// Character `H`
    CharH = 0x18,
    /// Character `I`
    CharI = 0x19,
    /// Character `J`
    CharJ = 0x1A,
    /// Character `K`
    CharK = 0x1B,
    /// Character `L`
    CharL = 0x1C,
    /// Character `M`
    CharM = 0x1D,
    /// Character `N`
    CharN = 0x1E,
    /// Character `O`
    CharO = 0x1F,
    /// Character `P`
    CharP = 0x20,
    /// Character `Q`
    CharQ = 0x21,
    /// Character `R`
    CharR = 0x22,
    /// Character `S`
    CharS = 0x23,
    /// Character `T`
    CharT = 0x24,
    /// Character `U`
    CharU = 0x25,
    /// Character `V`
    CharV = 0x26,
    /// Character `W`
    CharW = 0x27,
    /// Character `X`
    CharX = 0x28,
    /// Character `Y`
    CharY = 0x29,
    /// Character `Z`
    CharZ = 0x2A,
    /// Character `[`
    CharSqBOpen = 0x2B,
    /// Character `\`
    CharBackslash = 0x2C,
    /// Character `]`
    CharSqBClose = 0x2D,
    /// Character `^`
    CharCaret = 0x2E,
    /// Character `_`
    CharUnderscore = 0x2F,
    /// Character SPACE
    CharSpace = 0x30,
    /// Character `!`
    CharExcMrk = 0x31,
    /// Character `"`
    CharQuoteMrk = 0x32,
    /// Character `#`
    CharHash = 0x33,
    /// Character `$`
    CharDollar = 0x34,
    /// Character `%`
    CharPercent = 0x35,
    /// Character `&`
    CharAmp = 0x36,
    /// Character `'`
    CharApost = 0x37,
    /// Character `(`
    CharRBOpen = 0x38,
    /// Character `)`
    CharRBClose = 0x39,
    /// Character `*`
    CharAsterisk = 0x3A,
    /// Character `+`
    CharPlus = 0x3B,
    /// Character `,`
    ///
    /// (Renders as space)
    CharComma = 0x3C,
    /// Character `-`
    CharMinus = 0x3D,
    /// Character `.`
    ///
    /// (Renders as space)
    CharPeriod = 0x3E,
    /// Character `/`
    CharSlash = 0x3F,
    /// Character `0`
    CharZero = 0x40,
    /// Character `1`
    CharOne = 0x41,
    /// Character `2`
    CharTwo = 0x42,
    /// Character `3`
    CharThree = 0x43,
    /// Character `4`
    CharFour = 0x44,
    /// Character `5`
    CharFive = 0x45,
    /// Character `6`
    CharSix = 0x46,
    /// Character `7`
    CharSeven = 0x47,
    /// Character `8`
    CharEight = 0x48,
    /// Character `9`
    CharNine = 0x49,
    /// Character `:`
    CharColon = 0x4A,
    /// Character `;`
    CharSColon = 0x4B,
    /// Character `<`
    CharLess = 0x4C,
    /// Character `=`
    CharEqual = 0x4D,
    /// Character `>`
    CharLarger = 0x4E,
    /// Character `?`
    CharQestMrk = 0x4F,
    /// CGRAM address 0x0
    ///
    /// See [HCS12SS59T::set_char_pattern()](super::HCS12SS59T::set_cgram_pattern())
    Ram0 = 0x00,
    /// CGRAM address 0x1
    ///
    /// See [HCS12SS59T::set_char_pattern()](super::HCS12SS59T::set_cgram_pattern())
    Ram1 = 0x01,
    /// CGRAM address 0x2
    ///
    /// See [HCS12SS59T::set_char_pattern()](super::HCS12SS59T::set_cgram_pattern())
    Ram2 = 0x02,
    /// CGRAM address 0x3
    ///
    /// See [HCS12SS59T::set_char_pattern()](super::HCS12SS59T::set_cgram_pattern())
    Ram3 = 0x03,
    /// CGRAM address 0x4
    ///
    /// See [HCS12SS59T::set_char_pattern()](super::HCS12SS59T::set_cgram_pattern())
    Ram4 = 0x04,
    /// CGRAM address 0x5
    ///
    /// See [HCS12SS59T::set_char_pattern()](super::HCS12SS59T::set_cgram_pattern())
    Ram5 = 0x05,
    /// CGRAM address 0x6
    ///
    /// See [HCS12SS59T::set_char_pattern()](super::HCS12SS59T::set_cgram_pattern())
    Ram6 = 0x06,
    /// CGRAM address 0x7
    ///
    /// See [HCS12SS59T::set_char_pattern()](super::HCS12SS59T::set_cgram_pattern())
    Ram7 = 0x07,
    /// CGRAM address 0x8
    ///
    /// See [HCS12SS59T::set_char_pattern()](super::HCS12SS59T::set_cgram_pattern())
    Ram8 = 0x08,
    /// CGRAM address 0x9
    ///
    /// See [HCS12SS59T::set_char_pattern()](super::HCS12SS59T::set_cgram_pattern())
    Ram9 = 0x09,
    /// CGRAM address 0xA
    ///
    /// See [HCS12SS59T::set_char_pattern()](super::HCS12SS59T::set_cgram_pattern())
    RamA = 0x0A,
    /// CGRAM address 0xB
    ///
    /// See [HCS12SS59T::set_char_pattern()](super::HCS12SS59T::set_cgram_pattern())
    RamB = 0x0B,
    /// CGRAM address 0xC
    ///
    /// See [HCS12SS59T::set_char_pattern()](super::HCS12SS59T::set_cgram_pattern())
    RamC = 0x0C,
    /// CGRAM address 0xD
    ///
    /// See [HCS12SS59T::set_char_pattern()](super::HCS12SS59T::set_cgram_pattern())
    RamD = 0x0D,
    /// CGRAM address 0xE
    ///
    /// See [HCS12SS59T::set_char_pattern()](super::HCS12SS59T::set_cgram_pattern())
    RamE = 0x0E,
    /// CGRAM address 0xF
    ///
    /// See [HCS12SS59T::set_char_pattern()](super::HCS12SS59T::set_cgram_pattern())
    RamF = 0x0F,
}

impl From<char> for FontTable {
    /// Converts a [char] to a [FontTable] variant
    ///
    /// Characters not available are converted to [?](FontTable::CharQestMrk)
    fn from(value: char) -> Self {
        char_to_font_code(value).try_into().unwrap()
    }
}
impl TryFrom<u8> for FontTable {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // Since the font table is contiguous and has no gaps, transmute is used here for fast conversion and small code size.
        if value > 0x4F {
            Err(())
        } else {
            unsafe { Ok(core::mem::transmute(value)) }
        }
    }
}

pub(crate) fn char_to_font_code(c: char) -> u8 {
    if !c.is_ascii() {
        return 79;
    }
    match c {
        '@'..='_' => c as u8 - 48,
        ' '..='/' => c as u8 + 16,
        'a'..='z' => c as u8 - 80,
        '0'..='?' => c as u8 + 16,
        _ => 79,
    }
}
