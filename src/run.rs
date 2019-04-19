use crate::gpio;

pub fn run() {
    let gpio16 = gpio::Gpio::new(16);
    let is_input = gpio::Gpio::<gpio::Input>::from(gpio16);

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
