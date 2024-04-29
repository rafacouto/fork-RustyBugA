use hal_encoder::EncoderController;
use stm32f1xx_hal::pac::tim2;

pub struct QuadratureEncoder {
    tim: *const tim2::RegisterBlock,
    last_steps: usize,
}

impl QuadratureEncoder {
    // the middle value to avoid overflows
    const RESET_VALUE: u16 = u16::MAX / 2;

    pub fn new(tim: *const tim2::RegisterBlock) -> Self {
        unsafe {
            // auto-reload value to the maximum
            (*tim).arr.write(|w| w.bits(u16::MAX as u32));

            // quadrature encoder mode, input capture channels 1 & 2
            (*tim).ccmr1_input().write(|w| w.cc1s().ti1().cc2s().ti2());

            // up/down on TI1FP1+TI2FP2 edges depending on complementary input
            (*tim).smcr.write(|w| w.sms().bits(tim2::smcr::SMS_A::EncoderMode3 as u8));

            // initial value to the middle
            (*tim).cnt.write(|w| w.bits(QuadratureEncoder::RESET_VALUE as u32));
        }

        Self {
            tim,
            last_steps: QuadratureEncoder::RESET_VALUE as usize,
        }
    }

    // return the current number of steps
    pub fn get_steps(&self) -> u16 {
        unsafe {
            // read the counter register
            (*self.tim).cnt.read().cnt().bits() as u16
        }
    }

    // set the current number of steps
    pub fn set_steps(&mut self, steps: u16) {
        unsafe {
            // set the counter register
            (*self.tim).cnt.write(|w| w.bits(steps as u32));
        }
    }

    // enable the encoder
    pub fn enable(&mut self) {
        unsafe {
            // set the counter enable bit
            (*self.tim).cr1.write(|w| w.cen().set_bit());
        }
    }

    // disable the encoder
    pub fn disable(&mut self) {
        unsafe {
            // clear the counter enable bit
            (*self.tim).cr1.write(|w| w.cen().clear_bit());
        }
    }
}

impl EncoderController<16> for QuadratureEncoder {
    fn steps(&self) -> usize {
        self.get_steps() as usize
    }

    fn reset(&mut self) {
        self.set_steps(QuadratureEncoder::RESET_VALUE)
    }

    fn last_steps_ref(&mut self) -> &mut usize {
        &mut self.last_steps
    }
}
