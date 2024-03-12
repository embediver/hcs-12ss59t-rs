#![doc = include_str!("../README.md")]
#![no_std]

pub mod font;
pub use font::FontTable;

pub mod animation;

use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{self, SpiDevice};

const NUM_DIGITS: usize = 12;

#[repr(u8)]
#[allow(dead_code)]
enum Command {
    DCRamWrite = 0x10,
    CGRamWrite = 0x20,
    ADRamWrite = 0x30,
    DisplayDutySet = 0x50,
    NumDigitsSet = 0x60,
    Lights = 0x70,
}
#[repr(u8)]
#[allow(dead_code)]
enum Lights {
    Normal = 0x00,
    Off = 0x01,
    On = 0x02,
}

#[derive(Clone, Copy, Debug)]
pub enum Error<E: spi::Error> {
    Spi(E),
    Gpio,
    InvalidInput,
}
impl<E: spi::Error> From<E> for Error<E> {
    fn from(value: E) -> Self {
        Error::Spi(value)
    }
}

pub struct HCS12SS59T<SPI, RstPin, VdonPin, Delay, CsPin> {
    spi: SPI,
    n_reset: RstPin,
    n_vdon: Option<VdonPin>,
    delay: Delay,
    cs: CsPin,
}

impl<SPI, RstPin, VdonPin, Delay, CsPin> HCS12SS59T<SPI, RstPin, VdonPin, Delay, CsPin>
where
    SPI: SpiDevice,
    RstPin: OutputPin,
    VdonPin: OutputPin,
    CsPin: OutputPin,
    Delay: embedded_hal::delay::DelayNs,
{
    /// Constructs a new HCS12SS59T
    ///
    /// Initialization has to be done seperately by calling [init()](Self::init()).
    ///
    /// It is necessary to have a dedicated CS-Pin and a [Delay](embedded_hal::delay::DelayNs) due to timing restrictions of the HCS-12SS59T.
    pub fn new(
        spi: SPI,
        n_reset: RstPin,
        delay: Delay,
        n_vdon: Option<VdonPin>,
        cs: CsPin,
    ) -> Self {
        Self {
            spi,
            n_reset,
            n_vdon,
            delay,
            cs,
        }
    }

    pub fn destroy(self) -> (SPI, RstPin, Delay, Option<VdonPin>, CsPin) {
        (self.spi, self.n_reset, self.delay, self.n_vdon, self.cs)
    }

    /// Initialize the VFD display
    ///
    /// Resets the display, turns on the supply voltage and sets brightness to 7.
    pub fn init(&mut self) -> Result<(), Error<SPI::Error>> {
        self.n_reset.set_low().map_err(|_| Error::Gpio)?;
        self.delay.delay_us(25);
        self.n_reset.set_high().map_err(|_| Error::Gpio)?;
        self.delay.delay_us(5);

        self.vd_on()?;

        self.send_cmd(Command::NumDigitsSet, NUM_DIGITS as u8)?;
        self.send_cmd(Command::DisplayDutySet, 7)?;
        self.send_cmd(Command::Lights, Lights::Normal as u8)?;

        Ok(())
    }

    /// Turns the supply voltage off (if supply pin is configured)
    pub fn vd_off(&mut self) -> Result<(), Error<SPI::Error>> {
        if let Some(pin) = &mut self.n_vdon {
            pin.set_high().map_err(|_| Error::Gpio)?; // Display voltage OFF
        }
        Ok(())
    }

    /// Turns the supply voltage on (if supply pin is configured)
    pub fn vd_on(&mut self) -> Result<(), Error<SPI::Error>> {
        if let Some(pin) = &mut self.n_vdon {
            pin.set_low().map_err(|_| Error::Gpio)?; // Display voltage ON
        }
        Ok(())
    }

    /// Set the brightness (duty cycle) of the Display
    ///
    /// Turns the display off when brightness is `0` and on when brightness is `1..15`.
    pub fn brightness(&mut self, brightness: u8) -> Result<(), Error<SPI::Error>> {
        match brightness {
            0 => self.vd_off(),
            1..=15 => {
                self.vd_on()?;
                self.send_cmd(Command::DisplayDutySet, brightness)
            }
            _ => Err(Error::InvalidInput),
        }
    }

    /// Send one command byte with with four bits argument payload
    ///
    /// (The higher four bit specify the command, the lower four bit are the argument)
    fn send_cmd(&mut self, cmd: Command, arg: u8) -> Result<(), Error<SPI::Error>> {
        let arg = arg & 0x0F;
        let command = [cmd as u8 | arg];
        self.cs.set_low().map_err(|_| Error::Gpio)?;
        self.delay.delay_us(5);
        self.spi.write(&command)?;
        self.delay.delay_us(20);
        self.cs.set_high().map_err(|_| Error::Gpio)?;
        Ok(())
    }

    /// Write abritrary bytes to the display controller
    pub fn write_buf(&mut self, buf: &[u8]) -> Result<(), Error<SPI::Error>> {
        self.cs.set_low().map_err(|_| Error::Gpio)?;
        self.delay.delay_us(1);
        for byte in buf {
            self.spi.write(&[*byte])?;
            self.delay.delay_us(8);
        }
        self.delay.delay_us(12);
        self.cs.set_high().map_err(|_| Error::Gpio)?;
        Ok(())
    }

    /// Display a string
    ///
    /// Convenience method to avoid converting the string to a iterator first.
    /// Characters are mapped using the internal [FontTable].
    /// Strings are truncated to fit the display.
    pub fn display_str(&mut self, text: &str) -> Result<(), Error<SPI::Error>> {
        self.display(text.chars())
    }

    /// Write to the display RAM
    ///
    /// Displays the data, discarding unneded items.
    ///
    /// `From<char>` is implemented for [FontTable] so this method can
    /// be called with strings by calling [chars()](::core::primitive::str::chars) on the string.
    /// Alternatively [display_str](HCS12SS59T::display_str) does this for you.
    pub fn display<T>(&mut self, data: T) -> Result<(), Error<SPI::Error>>
    where
        T: IntoIterator,
        T::Item: Into<FontTable>,
    {
        let mut buf = [48_u8; NUM_DIGITS + 1];
        buf[0] = Command::DCRamWrite as u8;

        for (buf, c) in buf.iter_mut().skip(1).rev().zip(data.into_iter()) {
            *buf = c.into() as u8;
        }
        self.cs.set_low().map_err(|_| Error::Gpio)?;
        self.delay.delay_us(1);
        for byte in buf {
            self.spi.write(&[byte])?;
            self.delay.delay_us(8);
        }
        self.delay.delay_us(12);
        self.cs.set_high().map_err(|_| Error::Gpio)?;
        Ok(())
    }

    /// Write a single character to display RAM
    ///
    /// The HCS-12SS59T has 16 byte DCRAM, from which 0..12 are usable for the 12 connected digits.
    pub fn set_char<C: Into<FontTable>>(
        &mut self,
        addr: u8,
        char: C,
    ) -> Result<(), Error<SPI::Error>> {
        let addr = addr & 0x0F;
        let command = [Command::DCRamWrite as u8 | addr, char.into() as u8];

        self.cs.set_low().map_err(|_| Error::Gpio)?;
        self.delay.delay_us(1);
        for byte in command {
            self.spi.write(&[byte])?;
            self.delay.delay_us(8);
        }
        self.delay.delay_us(12);
        self.cs.set_high().map_err(|_| Error::Gpio)?;
        Ok(())
    }

    /// Set character generator RAM
    ///
    /// Write a two byte character pattern to one of 16 CGRAM adresses.
    ///
    /// Valid address values are [FontTable::Ram0] to [FontTable::RamF]
    ///
    /// The pattern is specified with two bytes for 16 segments,
    /// for a 14 segment display, segment 2 and 5 are don't care.
    ///
    /// |    Bit | 7     | 6     | 5     | 4     | 3     | 2     | 1     | 0    |
    /// |-------:|-------|-------|-------|-------|-------|-------|-------|------|
    /// | Byte 0 | SEG8  | SEG7  | SEG6  | SEG5  | SEG4  | SEG3  | SEG2  | SEG1 |
    /// | Byte 1 | SEG16 | SEG15 | SEG14 | SEG13 | SEG12 | SEG11 | SEG10 | SEG9 |
    ///
    /// ``` text
    ///   SEG1     SEG2
    /// S S     S     0 3
    /// E  E    E    1  G
    /// G   G   G   G   E
    /// 8    1  9  E    S
    ///       6   S
    ///   SEG15   SEG11
    /// S     4 S S     4
    /// E    1  E  E    G
    /// G   G   G   G   E
    /// 7  E    1    1  S
    ///   S     3     2
    ///   SEG6     SEG5
    /// ```
    pub fn set_cgram_pattern(
        &mut self,
        addr: FontTable,
        pattern: [u8; 2],
    ) -> Result<(), Error<SPI::Error>> {
        use FontTable::*;
        if !matches!(
            addr,
            Ram0 | Ram1
                | Ram2
                | Ram3
                | Ram4
                | Ram5
                | Ram6
                | Ram7
                | Ram8
                | Ram9
                | RamA
                | RamB
                | RamC
                | RamD
                | RamE
                | RamF
        ) {
            return Err(Error::InvalidInput);
        }
        let command = [
            Command::CGRamWrite as u8 | addr as u8,
            pattern[0],
            pattern[1],
        ];

        self.cs.set_low().map_err(|_| Error::Gpio)?;
        self.delay.delay_us(1);
        for byte in command {
            self.spi.write(&[byte])?;
            self.delay.delay_us(8);
        }
        self.delay.delay_us(12);
        self.cs.set_high().map_err(|_| Error::Gpio)?;
        Ok(())
    }
}
