use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
use pc_keyboard::{ Keyboard, layouts::Us104Key, ScancodeSet1, HandleControl, DecodedKey };
use crate::{ println, print };
use crate::cpu::gdt;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        // This is unsafe because it cannot guarantee that the index is actually valid
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptorIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptorIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

// Sends a "end of interrupt" (EOI) signal to the PIC
// this should be called at the end of every interrupt
fn pics_eoi(i: InterruptorIndex) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(i.as_u8());
    }
}

pub fn init_pic() {
    unsafe { PICS.lock().initialize() };
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum InterruptorIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptorIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }
    fn as_usize(self) -> usize {
        self as usize
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    //println!("A timer interrupt has been fired!");
    pics_eoi(InterruptorIndex::Timer);
}
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: spin::Mutex<Keyboard<Us104Key, ScancodeSet1>> = {
            spin::Mutex::new(Keyboard::new(Us104Key, ScancodeSet1, HandleControl::Ignore))
        };
    };

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    {
        let mut keyboard = KEYBOARD.lock();
        if let Ok(Some(keyevent)) = keyboard.add_byte(scancode) {
            if let Some(decodedkey) = keyboard.process_keyevent(keyevent) {
                match decodedkey {
                    DecodedKey::RawKey(keycode) => print!("{:?}", keycode),
                    DecodedKey::Unicode(char) => print!("{}", char),
                }
            }
        }
    }

    pics_eoi(InterruptorIndex::Keyboard);
}

#[test_case]
fn test_breakpoint_exception() {
    // Invoke the breakpoint exception
    x86_64::instructions::interrupts::int3();
    // If execution continues, all is good.
}
