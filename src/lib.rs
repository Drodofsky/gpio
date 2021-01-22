#![no_std]
use core::ptr;

pub trait Port {
    fn get_addr(&self) -> u32;
}

pub struct PinOut<P, const N: u8>
where
    P: Port,
{
    port: P,
}

pub struct PinIn<P, const N: u8>
where
    P: Port,
{
    port: P,
}

impl<P, const N: u8> PinIn<P, N>
where
    P: Port,
{
    fn new(port: P) -> Self {
        Self { port }
    }
    pub fn read(&self) -> u32 {
        let gpiox_idr = (self.port.get_addr() + 0x10) as *const u32;
        unsafe { (ptr::read_volatile(gpiox_idr) >> N) & 1 }
    }
    pub fn set_pull_up(&mut self) {
        let gpiox_pupdr = (self.port.get_addr() + 0x0C) as *mut u32;
        unsafe {
            ptr::write_volatile(gpiox_pupdr, ptr::read(gpiox_pupdr) | (1 << (N * 2)));
            ptr::write_volatile(
                gpiox_pupdr,
                ptr::read(gpiox_pupdr) & (!(1 << ((N * 2) + 1))),
            );
        }
    }
    pub fn set_pull_down(&mut self) {
        let gpiox_pupdr = (self.port.get_addr() + 0x0C) as *mut u32;
        unsafe {
            ptr::write_volatile(gpiox_pupdr, ptr::read(gpiox_pupdr) & (!(1 << (N * 2))));
            ptr::write_volatile(gpiox_pupdr, ptr::read(gpiox_pupdr) | (1 << ((N * 2) + 1)));
        }
    }
    pub fn reset(&mut self) {
        let gpiox_pupdr = (self.port.get_addr() + 0x0C) as *mut u32;
        unsafe {
            ptr::write_volatile(gpiox_pupdr, ptr::read(gpiox_pupdr) & (!(1 << (N * 2))));
            ptr::write_volatile(
                gpiox_pupdr,
                ptr::read(gpiox_pupdr) & (!(1 << ((N * 2) + 1))),
            );
        }
    }
}

impl<P: Port, const N: u8> PinOut<P, N> {
    fn new(p: P) -> Self {
        let moder = p.get_addr() as *mut u32;
        unsafe {
            ptr::write_volatile(moder, ptr::read(moder) | (1 << (N * 2)));
            ptr::write_volatile(moder, ptr::read(moder) & (!(1 << ((N * 2) + 1))));
        }
        Self { port: p }
    }
    pub fn set(&mut self) {
        let bsrr = (self.port.get_addr() + 0x18) as *mut u32;
        unsafe { ptr::write_volatile(bsrr, ptr::read(bsrr) | (1 << (N))) };
    }

    pub fn reset(&mut self) {
        let bsrr = (self.port.get_addr() + 0x18) as *mut u32;
        unsafe { ptr::write_volatile(bsrr, ptr::read(bsrr) | (1 << (N + 16))) };
    }
}

macro_rules! create_port {
    ($name:ident,$addr:tt,$clock:tt) => {
        pub struct $name {}
        impl Port for $name {
            fn get_addr(&self) -> u32 {
                $addr
            }
        }
        impl $name {
            pub fn pin_in<const N: u8>() -> PinIn<$name, N> {
                PinIn::new($name {})
            }
            pub fn pin_out<const N: u8>() -> PinOut<$name, N> {
                PinOut::new($name {})
            }
            pub fn init() {
                let rcc_ahbenr = (0x4002_1000 + 0x14) as *mut u32;
                unsafe { ptr::write_volatile(rcc_ahbenr, ptr::read(rcc_ahbenr) | (1 << $clock)) };
            }
        }
    };
}

create_port!(PortA, 0x4800_0000, 17);
create_port!(PortB, 0x4800_0400, 18);
create_port!(PortC, 0x4800_0800, 19);
create_port!(PortD, 0x4800_0C00, 20);
create_port!(PortE, 0x4800_1000, 21);
create_port!(PortF, 0x4800_1400, 22);
