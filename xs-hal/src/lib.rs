//! XiangShan Hal Implementation
#![no_std]

extern crate core;
extern crate register;

use core::mem::replace;
#[allow(unused_imports)]
use register::{mmio::*, register_bitfields, register_structs};

pub const UARTLITE_MMIO: usize = 0x4060_0000;

register_structs! {
    pub UartLite {
        (0x00 => rx_fifo: ReadOnly<u32>),
        (0x04 => tx_fifo: ReadWrite<u32>),
        (0x08 => stat_reg: ReadOnly<u32>),
        (0x0c => ctrl_reg: ReadWrite<u32>),
        (0x10 => @END),
    }
}

impl UartLite {
    pub fn putchar(&mut self, _ch: u8) {
        // TODO
    }

    pub fn getchar(&self) -> Result<u8, ()> {
        // TODO
        Err(())
    }
}

pub struct XSPeripherals {
    uart_lite: Option<&'static mut UartLite>
}

impl XSPeripherals {

    pub fn take_uart_lite(&mut self) -> &'static mut UartLite {
        let uart = replace(&mut self.uart_lite, None);
        uart.unwrap()
    }
}