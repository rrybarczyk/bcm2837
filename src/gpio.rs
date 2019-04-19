use volatile_register::{RW, RO, WO};
use crate::common::PERIPHERAL_BASE;

/// 0x7E20 0000 GPIO Base Address
pub const GPIO_BASE: usize  = PERIPHERAL_BASE + 0x200000;
/// 0x07, bitmask
const GPIO_FSEL_MASK: u8 = 0b111;

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[repr(C)]
struct Registers {
    /// 0x7E20 0000 - GPIO Function Select 0
    /// 0x7E20 0004 - GPIO Function Select 1
    /// 0x7E20 0008 - GPIO Function Select 2
    /// 0x7E20 000C - GPIO Function Select 3
    /// 0x7E20 0010 - GPIO Function Select 4
    /// 0x7E20 0014 - GPIO Function Select 5
    GPFSEL: [RW<u32>; 6],

    /// 0x7E20 0018 Reserved
    reserved0: [u32; 1],

    /// 0x7E20 001C GPIO Pin Output Set 0
    /// 0x7E20 0020 GPIO Pin Output Set 1
    GPSET: [WO<u32>; 2],

    /// 0x7E20 0024 Reserved
    reserved1: [u32; 1],

    /// 0x7E20 0028 GPIO Pin Clear 0
    /// 0x7E20 002C GPIO Pin Clear 1
    GPCLR: [WO<u32>; 2],

    /// 0x7E20 0030 Reserved
    reserved2: [u32; 1],

    /// 0x7E20 0034 GPIO Pin Level 0
    /// 0x7E20 0038 GPIO Pin Level 1
    GPLEV: [RO<u32>; 2],

    /// 0x7E20 003C Reserved
    reserved3: [u32; 1],

    /// 0x7E20 0040 GPIO Pin Event Detect Status 0
    /// 0x7E20 0044 GPIO Pin Even Detect Status 1
    GPEDS: [RW<u32>; 2],

    /// 0x7E20 0048 Reserved
    reserved4: [u32; 1],

    /// 0x7E20 004C GPIO Pin Rising Edge Detect Enable 0
    /// 0x7E20 0050 GPIO Pin Rising Edge Detect Enable 1
    GPREN: [RW<u32>; 2],

    /// 0x7E20 0054 Reserved
    reserved5: [u32; 1],

    /// 0x7E20 0058 GPIO Pin Falling Edge Detect Enable 0
    /// 0x7E20 005C GPIO Pin Falling Edge Detect Enable 1
    GPFEN: [RW<u32>; 2],

    /// 0x7E20 0060 Reserved
    reserved6: [u32; 1],

    /// 0x7E20 0064 GPIO Pin High Detect Enable 0
    /// 0x7E20 0068 GPIO Pin High Detect Enable 1
    GPHEN: [RW<u32>; 2],

    /// 0x7E20 006C Reserved
    reserved7: [u32; 1],

    /// 0x7E20 0070 GPIO Pin Low Detect Enable 0
    /// 0x7E20 0074 GPIO Pin Low Detect Enable 1
    GPLEN: [RW<u32>; 2],

    /// 0x7E20 0078 Reserved
    reserved8: [u32; 1],

    /// 0x7E20 007C GPIO Pin Async. Rising Edge Detect 0
    /// 0x7E20 0080 GPIO Pin Async. Rising Edge Detect 1
    GPAREN: [RW<u32>; 2],

    /// 0x7E20 0084 Reserved
    reserved9: [u32; 1],

    /// 0x7E20 0088 GPIO Pin Async. Falling Edge Detect 0
    /// 0x7E20 008C GPIO Pin Async. Falling Edge Detect 1
    GPAFEN: [RW<u32>; 2],

    /// 0x7E20 0090 Reserved
    reserved10: [u32; 1],

    ///  0x7E20 0094 GPIO Pin Pull-up/down Enable
    GPPUD: [RW<u32>; 1],

    /// 0x7E20 0098 GPIO Pin Pull-up/down Enable Clock 0
    /// 0x7E20 009C GPIO Pin Pull-up/down Enable Clock 1
    GPPUDCLK: [RW<u32>; 2],

    /// 0x7E20 00A0 Reserved
    reserved11: [u32; 1],
}

/// Function Select Control Registers
/// GPIO_PIN / 10 + ((GPIO_PIN % 10) * 3)
#[allow(non_camel_case_types)]
pub enum FunctionSelectMode {
    GPIO_FSEL_INPUT     = 0b000,        // 0x00, Input
    GPIO_FSEL_OUTPUT    = 0b001,        // 0x01, Output
    GPIO_FSEL_ALT0      = 0b100,        // 0x04, Alternate Function 0
    GPIO_FSEL_ALT1      = 0b101,        // 0x05, Alternate Function 1
    GPIO_FSEL_ALT2      = 0b110,        // 0x06, Alternate Function 2
    GPIO_FSEL_ALT3      = 0b111,        // 0x07, Alternate Function 3
    GPIO_FSEL_ALT4      = 0b011,        // 0x03, Alternate Function 4
    GPIO_FSEL_ALT5      = 0b010,        // 0x02, Alternate Function 5
}


pub struct Gpio<S> {
    pin: u8,
    registers: &'static mut Registers,
    state: S,
}
pub struct Alt { }
pub struct Input { }
pub struct Output { }
pub struct Uninitialized { }

impl Gpio<Uninitialized> {
    pub fn new(pin: u8) -> Self {
        Gpio {
            pin, 
            registers: unsafe { &mut *(GPIO_BASE as *mut Registers) },
            state: Uninitialized { }
        }
    }
}

impl From<Gpio<Uninitialized>> for Gpio<Input> {
    fn from(gpio: Gpio<Uninitialized>) -> Gpio<Input> { 
        let bank = (gpio.pin / 10) as usize;
        let bits = ((gpio.pin % 10) * 3) as usize;

        let mask = !(0b111 << bits);
        unsafe { gpio.registers.GPFSEL[bank].write(gpio.registers.GPFSEL[bank].read() & mask); }

        let mask = (FunctionSelectMode::GPIO_FSEL_INPUT as u32) << bits;
        unsafe { gpio.registers.GPFSEL[bank].write(gpio.registers.GPFSEL[bank].read() | mask); }

        Gpio {
            pin: gpio.pin,
            registers: gpio.registers,
            state: Input { },
        }
    }
}

impl From<Gpio<Uninitialized>> for Gpio<Output> {
    fn from(gpio: Gpio<Uninitialized>) -> Gpio<Output> { 
        let bank = (gpio.pin / 10) as usize;
        let bits = ((gpio.pin % 10) * 3) as usize;

        let mask = !(0b111 << bits);
        unsafe { gpio.registers.GPFSEL[bank].write(gpio.registers.GPFSEL[bank].read() & mask); }

        let mask = (FunctionSelectMode::GPIO_FSEL_OUTPUT as u32) << bits;
        unsafe { gpio.registers.GPFSEL[bank].write(gpio.registers.GPFSEL[bank].read() | mask); }

        Gpio {
            pin: gpio.pin,
            registers: gpio.registers,
            state: Output { },
        }
    }
}

impl From<Gpio<Uninitialized>> for Gpio<Alt> {
    fn from(gpio: Gpio<Uninitialized>) -> Gpio<Alt> { 
        unimplemented!();
    }
}

impl Gpio<Alt> {
    pub fn into_alt(self, altfunc: FunctionSelectMode) -> Gpio<Alt> {
        let bank = (self.pin / 10) as usize;
        let bits = ((self.pin % 10) * 3) as usize;

        let mask = !(0b111 << bits);
        unsafe { self.registers.GPFSEL[bank].write(self.registers.GPFSEL[bank].read() & mask); }

        // let mask = (FunctionSelectMode::GPIO_FSEL_OUTPUT as u32) << bits;
        let mask = (altfunc as u32) << bits;
        unsafe { self.registers.GPFSEL[bank].write(self.registers.GPFSEL[bank].read() | mask); }

        Gpio {
            pin: self.pin,
            registers: self.registers,
            state: Alt { },
        }

    }

    fn set_fsel(value: Gpio<Alt>) -> Self { unimplemented!(); }
}

impl Gpio<Input> {
    fn is_high(&mut self) -> bool { 
        let bank = (self.pin / 32) as usize;
        let bit = (self.pin % 32) as usize;
        let mask = (self.pin as u32) << bit;
        unsafe { (self.registers.GPLEV[bank].read() & mask) == mask }
    }

    fn is_low(&mut self) -> bool { !self.is_high() }
}

impl Gpio<Output> {
    fn clear(&mut self) { 
        let bank = (self.pin / 32) as usize;
        let bit = (self.pin % 32) as usize;
        unsafe { self.registers.GPSET[bank].write(1 << bit); }
    }

    fn set(&mut self) { 
        let bank = (self.pin / 32) as usize;
        let bit = (self.pin % 32) as usize;
        unsafe { self.registers.GPCLR[bank].write(1 << bit); }
    }
}
