extern crate libc;

mod sim_avr;
mod sim_elf;
mod stdint;

use std::ffi::CString;
use std::ffi::CStr;
use sim_avr::Struct_avr_t;
use sim_avr::Struct_avr_irq_pool_t;
pub use sim_avr::CPUState;
use sim_avr::avr_run;
use sim_elf::avr_load_firmware;
use sim_elf::Struct_elf_firmware_t;
use sim_elf::elf_read_firmware;
use std::default::Default;
use sim_avr::avr_irq_notify_t;
use sim_avr::Struct_avr_irq_t;
use sim_avr::avr_irq_register_notify;
use sim_avr::avr_io_getirq;
use sim_avr::avr_iomem_getirq;
use sim_avr::avr_alloc_irq;
use sim_avr::avr_connect_irq;
use sim_avr::avr_raise_irq;
use std::ptr;
use std::mem;


pub struct Avr {
    avr: Struct_avr_t,
}

pub struct AvrIrq {
    irq: *mut Struct_avr_irq_t
}

impl Avr {
    pub fn init(name: &str) -> Avr {
        let name = CString::new(name).unwrap();
        
        Avr {
            avr : unsafe {
                let avr = sim_avr::avr_make_mcu_by_name(name.as_ptr());

                sim_avr::avr_init(avr);
                *avr
            }
        }
    }

    pub fn set_trace(&mut self, trace: bool) {
        self.avr._bindgen_bitfield_1_ = trace as u8;
        //self.avr._bindgen_bitfield_2_ = 3 as u8;
    }

    pub fn load_firmware(&mut self, firmware: &mut AvrFirmware) {
        unsafe {
            avr_load_firmware(&mut self.avr as *mut Struct_avr_t, &mut firmware.firmware as *mut Struct_elf_firmware_t);
        }
    }

    pub fn run(&mut self) -> CPUState {
        unsafe {
            avr_run(&mut self.avr as *mut Struct_avr_t)
        }
    }

    unsafe extern "C" fn notify_handler(irq: *mut Struct_avr_irq_t,
                                 value: stdint::uint32_t,
                                 param: *mut ::std::os::raw::c_void) {
    let closure: &mut Box<FnMut(&AvrIrq, u32) -> ()> = unsafe { mem::transmute(param) };
        unsafe {
            closure(&AvrIrq { irq: irq }, value as u32);
        }
    }


    pub fn register_notify<'a, F>(&'a mut self, avr_irq: &mut AvrIrq, mut notify: F) 
        where F: FnMut(&AvrIrq, u32) -> (), F: 'a 
    {
        let cb: Box<Box<FnMut(&AvrIrq, u32) -> ()>> = Box::new(Box::new(notify));
        unsafe {
            avr_irq_register_notify(avr_irq.irq, Some(Avr::notify_handler), Box::into_raw(cb) as *mut _);
        }
    }

    pub fn io_get_irq(&mut self, ctl: u32, index: i32) -> AvrIrq {
        unsafe {
            let irq = avr_io_getirq(&mut self.avr as *mut Struct_avr_t, ctl, index);
            println!("{}", CStr::from_ptr((*irq).name).to_str().unwrap());
            AvrIrq { irq: irq }
        }
    }

    pub fn ioport_getirq(&mut self, name: char, index: i32) -> AvrIrq {
        let ctl = Avr::ioctl_ioport_getirq(name);
        self.io_get_irq(ctl, index)
    }

    pub fn iomem_get_irq(&mut self,  addr: u16,
                         name: &str,
                         index: i32) -> AvrIrq {
        let name = CString::new(name).unwrap();

        unsafe {
            let irq = avr_iomem_getirq(&mut self.avr as *mut Struct_avr_t, 
                                       addr,
                                       name.as_ptr(), index);
            println!("{}", CStr::from_ptr((*irq).name).to_str().unwrap());
            AvrIrq { irq: irq }
        }
    }

    pub fn ioctl_ioport_getirq(name: char) -> u32 {
        ('i' as u32) << 24 | 
            ('o' as u32) << 16 | 
            ('g' as u32) << 8 | (name as u32)
    }

    pub fn ioctl_timer_getirq(name: char) -> u32 {
        ('t' as u32) << 24 | 
            ('m' as u32) << 16 | 
            ('r' as u32) << 8 | (name as u32)
    }

    pub fn alloc_irq(&mut self, index: u32, count: u32, names: &str) -> AvrIrq {
        let mut names = CString::new(names).unwrap();
        AvrIrq {
            irq: unsafe {
                avr_alloc_irq(&mut self.avr.irq_pool as *mut Struct_avr_irq_pool_t, 
                              index, count, &mut names.as_ptr() as *mut *const i8)
            }
        }
    }

    pub fn connect_irq(src: &mut AvrIrq, dst: &mut AvrIrq) {
        unsafe {
            avr_connect_irq(src.irq, dst.irq);
        }
    }


}

pub struct AvrFirmware {
    firmware: Struct_elf_firmware_t
}

impl AvrFirmware {
    pub fn load(name: &str) -> AvrFirmware {
        let name = CString::new(name).unwrap();
        unsafe {
            let mut firmware : Struct_elf_firmware_t = Default::default();
            elf_read_firmware(name.as_ptr(), &mut firmware as *mut Struct_elf_firmware_t);
            AvrFirmware { firmware: firmware }
        }
    }
}

impl AvrIrq {
    pub fn raise(&mut self, value: u32) {
        unsafe {
            avr_raise_irq(self.irq, value)
        }
    }

    pub fn name(&self) -> &str {
        unsafe {
            CStr::from_ptr((*self.irq).name).to_str().unwrap()
        }
    }
}

