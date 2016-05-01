extern crate simavr;

use simavr::Avr;
use simavr::AvrFirmware;
use simavr::CPUState;

#[test]
#[ignore]
fn it_works() {
    let mut avr = Avr::init("atmega328p");
    let mut avr_firmware = AvrFirmware::load("/home/alex/scratch/platformio_test/.pioenvs/328p16m/firmware.elf");
    avr.load_firmware(&mut avr_firmware);
    //avr.set_trace(true);

    {
        let mut irq = avr.iomem_get_irq(37, "wat", 1);
        avr.register_notify(irq, move |irq, value| { println!("io37 {}", value)});

        for i in 0..8 {
            let ioport = Avr::ioctl_ioport_getirq('D');
            let mut irq = avr.io_get_irq(ioport, i);

            avr.register_notify(&mut irq, 
                                move |irq, value| { println!("PORTB {} {}", i, value)});
        }

        for i in 0..3 {
            let mut ioport = Avr::ioctl_timer_getirq((48 + i) as u8 as char);
            let mut irq = avr.io_get_irq(ioport, 0);
            avr.register_notify(&mut irq, 
                                move |irq, value| { println!("TIMER {} {}", i, value)});
        }

        //let irq = Avr::alloc_irq(0, 0, "test");
        let ioport = Avr::ioctl_ioport_getirq('B');
        let mut irq = avr.io_get_irq(ioport, 4);
        avr.register_notify(&mut irq, 
                            move |irq, value| { println!("REG {} {}", 4, value)});
    }
    //irq.raise(10);

    let mut counter = 0;
    loop {
        counter += 1;
        //if counter == 1000000 {
        //    break;
        //}

        match avr.run() {
            CPUState::cpu_Done => break,
            CPUState::cpu_Crashed => println!("Crashed!"),
            _ => ()
        }
    }
}

use std::cell::RefCell;
#[test]
fn fridge() {
    let mut avr = Avr::init("atmega328p");
    //let mut avr_firmware = AvrFirmware::load("/home/alex/projects/fridge/.pioenvs/uno/firmware.elf");
    let mut avr_firmware = AvrFirmware::load("/home/alex/scratch/platformio_test/.pioenvs/uno/firmware.elf");
    avr.load_firmware(&mut avr_firmware);
    let state = avr.run();

    assert!(state == CPUState::cpu_Running);

    {
    let adc_irq = ('a' as u32) << 24 | 
            ('d' as u32) << 16 | 
            ('c' as u32) << 8 | (' ' as u32);
    let mut irq_out_trigger = avr.io_get_irq(adc_irq, 18);
    let mut irq_in_trigger = avr.io_get_irq(adc_irq, 17);
    let mut t1_irq = avr.io_get_irq(adc_irq, 0);
    let mut t2_irq = avr.io_get_irq(adc_irq, 1);
    let mut t3_irq = avr.io_get_irq(adc_irq, 2);
    avr.register_notify(&mut irq_out_trigger, 
                        |irq, value| { 
                            //println!("ADC trigger: {} {:32b}", irq.name(), value);
                            t1_irq.raise(200);
                            t2_irq.raise(200);
                            t3_irq.raise(200);
                            irq_in_trigger.raise(0);
                        });


    let mut ioport = Avr::ioctl_timer_getirq('0');
    let mut irq = avr.io_get_irq(ioport, 0);
    avr.register_notify(&mut irq, 
                        move |irq, value| { println!("COLD FAN = {}", value)});

    let mut irq = avr.ioport_getirq('D', 3);
    avr.register_notify(&mut irq, 
                        move |irq, value| { println!("TEC = {}", value)});

    //let mut irq = avr.ioport_getirq('D', 6);
    //avr.register_notify(&mut irq, 
    //                    move |irq, value| { println!("HOT FAN = {}", value)});

//    let mut irq = avr.ioport_getirq('D', 3);
//    avr.register_notify(&mut irq, 
//                        move |irq, value| { println!("COLD FAN = {}", value)});

    let mut irq = avr.ioport_getirq('B', 1);
    avr.register_notify(&mut irq, 
                        move |irq, value| { println!("RED LED = {}", value)});

    } 

    //avr.set_trace(true);

    loop {
        avr.run(); 
    }
}
