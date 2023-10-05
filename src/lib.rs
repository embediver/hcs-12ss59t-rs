#![no_std]

use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiDevice;

const NUM_DIGITS: usize = 12;

#[repr(u8)]
enum Command {
    DCRamWrite = 0x10,
    CGRamWrite = 0x20,
    ADRamWrite = 0x30,
    DisplayDutySet = 0x50,
    NumDigitsSet = 0x60,
    Lights = 0x70,
}
#[repr(u8)]
enum Lights {
    Normal = 0x00,
    Off = 0x01,
    On = 0x02,
}

#[derive(Clone, Copy, Debug)]
pub enum Error {
    Spi,
    Gpio,
    InvalidInput,
}

pub struct HCS12SS59T<SPI, RstPin, VdonPin, Delay> {
    spi: SPI,
    n_reset: RstPin,
    n_vdon: Option<VdonPin>,
    delay: Delay,
}

impl<SPI, RstPin, VdonPin, Delay> HCS12SS59T<SPI, RstPin, VdonPin, Delay>
where
    SPI: SpiDevice,
    RstPin: OutputPin,
    VdonPin: OutputPin,
    Delay: embedded_hal::delay::DelayUs,
{
    /// Constructs a new HCS12SS59T
    ///
    /// Initialization has to be done seperately by calling [init()](Self::init()).
    pub fn new(spi: SPI, n_reset: RstPin, delay: Delay, n_vdon: Option<VdonPin>) -> Self {
        Self {
            spi,
            n_reset,
            n_vdon,
            delay,
        }
    }

    pub fn destroy(self) -> (SPI, RstPin, Delay, Option<VdonPin>) {
        (self.spi, self.n_reset, self.delay, self.n_vdon)
    }

    /// Initialize the VFD display
    ///
    /// Resets the display, turns on the supply voltage and sets brightness to 7.
    pub fn init(&mut self) -> Result<(), Error> {
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
    pub fn vd_off(&mut self) -> Result<(), Error> {
        if let Some(pin) = &mut self.n_vdon {
            pin.set_high().map_err(|_| Error::Gpio)?; // Display voltage OFF
        }
        Ok(())
    }

    /// Turns the supply voltage on (if supply pin is configured)
    pub fn vd_on(&mut self) -> Result<(), Error> {
        if let Some(pin) = &mut self.n_vdon {
            pin.set_low().map_err(|_| Error::Gpio)?; // Display voltage ON
        }
        Ok(())
    }

    /// Set the brightness (duty cycle) of the Display
    ///
    /// Turns the display off when brightness is `0` and on when brightness is `1..15`.
    pub fn brightness(&mut self, brightness: u8) -> Result<(), Error> {
        match brightness {
            0 => self.vd_off(),
            1..=15 => {
                self.vd_on()?;
                self.send_cmd(Command::DisplayDutySet, brightness)
            }
            _ => Err(Error::InvalidInput),
        }
    }

    fn send_cmd(&mut self, cmd: Command, arg: u8) -> Result<(), Error> {
        let arg = arg & 0x0F;
        let command = [cmd as u8 | arg];
        self.spi.write(&command).map_err(|_| Error::Spi)?;
        Ok(())
    }

    pub fn display(&mut self, text: &str) -> Result<(), Error> {
        let mut data = [0_u8; NUM_DIGITS + 1];
        data[0] = Command::DCRamWrite as u8;

        for (data, c) in data.iter_mut().zip(text.chars().into_iter()) {
            *data = char_to_font_code(c);
        }
        self.spi.write(&data).map_err(|_| Error::Spi)
    }
}

fn char_to_font_code(c: char) -> u8 {
    if !c.is_ascii() {
        return 79;
    }
    match c {
        '@'..='_' => c as u8 - 48,
        ' '..='/' => c as u8 + 16,
        'a'..='z' => c as u8 - 80,
        _ => 79,
    }
}
