#![allow(clippy::suspicious_op_assign_impl)]
#![allow(clippy::suspicious_arithmetic_impl)]

use core::cell::UnsafeCell;
use core::marker::PhantomData;
use tock_registers::registers::{IntLike, RegisterLongName};

const PFC_PMMR: *mut u32 = 0xE6060000 as *mut u32; // R/W	32	LSI Multiplexed Pin Setting Mask Register

#[repr(transparent)]
pub struct PfcReadWrite<T: IntLike, R: RegisterLongName = ()> {
    value: UnsafeCell<T>,
    associated_register: PhantomData<R>,
}

impl<R: RegisterLongName> PfcReadWrite<u32, R> {
    #[inline]
    /// Get the raw register value
    pub fn get(&self) -> u32 {
        unsafe { ::core::ptr::read_volatile(self.value.get()) }
    }

    #[inline]
    /// Set the raw register value
    pub fn set(&self, value: u32) {
        unsafe {
            ::core::ptr::write_volatile(PFC_PMMR, !value);
            ::core::ptr::write_volatile(self.value.get(), value)
        }
    }
}
