extern crate simavr;

use simavr::Avr;
use simavr::AvrFirmware;
use simavr::CPUState;

#[test]
fn it_works() {
    let mut avr = Avr::init("atmega328p");
    //let mut avr_firmware = AvrFirmware::load("/home/alex/projects/simavr/examples/board_ledramp/atmega48_ledramp.axf");
    let mut avr_firmware = AvrFirmware::load("/home/alex/scratch/platformio_test/.pioenvs/328p16m/firmware.elf");
    avr.load_firmware(&mut avr_firmware);
    //let state = avr.run();
    //assert_eq!(state, CPUState::cpu_Running);

    let mut irq = avr.iomem_get_irq(37, "wat", 1);
    avr.register_notify(&mut irq, 
                           move |irq, value| { println!("io37 {}", value)});

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

    irq.raise(10);


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

