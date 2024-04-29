#![no_std]
#![cfg_attr(not(doc), no_main)]
#[warn(dead_code)]
use core::convert::Infallible;
use core::ops::Shl;
use mightybuga_bsc as board;
use mightybuga_bsc::prelude::*;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = board::Mightybuga_BSC::take().unwrap();
    let mut delay = board.delay;
    let mut serial = board.serial;

    let mut encoder_l = board.encoder_l;
    encoder_l.enable();
    let mut encoder_r = board.encoder_r;
    encoder_r.enable();

    print(&mut serial, "move the encoders!\r\n");
    let mut last: u32 = 0;
    loop {
        let steps_l = encoder_l.get_steps();
        let steps_r = encoder_r.get_steps();
        let next: u32 = (steps_l as u32).shl(16) | (steps_r as u32);
        if last != next {
            print(&mut serial, "steps: left=");
            print_number(&mut serial, steps_l as u32);
            print(&mut serial, " right=");
            print_number(&mut serial, steps_r as u32);
            print(&mut serial, "\r\n");
            last = next;
        }

        // don't burn the CPU
        delay.delay(100.millis());
    }
}

fn print<'a>(output: &'a mut dyn embedded_hal::serial::Write<u8, Error = Infallible>, s: &str) {
    s.chars().for_each(|c| {
        let _ = nb::block!(output.write(c as u8));
    });
}

fn print_number<'a>(
    output: &'a mut dyn embedded_hal::serial::Write<u8, Error = Infallible>,
    n: u32,
) {
    let mut buffer = [b'0'; 10];
    let mut i = 0;
    let mut n = n;
    while n > 0 {
        buffer[i] = (n % 10) as u8 + b'0';
        n /= 10;
        i += 1;
    }
    if i == 0 {
        i += 1;
    }
    while i > 0 {
        i -= 1;
        let _ = nb::block!(output.write(buffer[i]));
    }
}
