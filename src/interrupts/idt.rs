use bitfield::bitfield;
use lazy_static::lazy_static;

/// Type of interrupt gate
#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum GateType {
    TaskGate32 = 0x5,
    InterruptGate16 = 0x6,
    TrapGate16 = 0x7,
    InterruptGate32 = 0xE,
    TrapGate32 = 0xF,
}

impl From<u8> for GateType {
    fn from(value: u8) -> GateType {
        match value {
            0x5 => GateType::TaskGate32,
            0x6 => GateType::InterruptGate16,
            0x7 => GateType::TrapGate16,
            0xE => GateType::InterruptGate32,
            0xF => GateType::TrapGate32,
            _ => panic!("Invalid conversion from u8 to IDT GateType: {:X}", value),
        }
    }
}

bitfield! {
    /// Entry in the IDT (Interrupt Descriptor Table)
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    struct IdtEntry(u64);
    impl Debug;
    u16, offset_low, set_offset_low: 15, 0;
    u16, selector, set_selector: 31, 16;
    // Bits 32 to 39 are unused
    u8, into GateType, gate_type, set_gate_type: 43, 40;
    bool, storage_segment, set_storage_segment: 44;
    u8, privilege, set_privilege: 46, 45;
    bool, present, set_present: 47;
    u16, offset_high, set_offset_high: 63, 48;
}

impl IdtEntry {
    /// Creates and initializes a new IdtEntry
    fn new(base: u32, selector: u8, gate_type: GateType, privilege: u8) -> IdtEntry {
        let mut entry = IdtEntry(0);
        entry.set_offset_low((base & 0xFF) as u16);
        entry.set_offset_high((base >> 16 & 0xFF) as u16);
        entry.set_gate_type(gate_type as u8);
        if gate_type != GateType::TaskGate32 {
            entry.set_storage_segment(true);
        }
        entry.set_privilege(privilege);
        entry.set_present(false);
        entry
    }

    /// Enable the interrupt
    fn enable(&mut self) {
        self.set_present(true)
    }

    /// Disable the interrupt
    fn disable(&mut self) {
        self.set_present(false)
    }
}

const IDT_ENTRIES_COUNT: usize = 32;
type IdtTable = [IdtEntry; IDT_ENTRIES_COUNT];

lazy_static! {
    /// Global Interrupt Descriptor Table
    static ref IDT_TABLE: IdtTable = [IdtEntry(0); IDT_ENTRIES_COUNT];
}

// Assembly routine loading the IDT table
extern "C" {
    fn load_IDT(offset: u32, limit: u16);
}

/// Initializes the IDT table and loads it into memory
pub fn init_idt() {
    unsafe {
        load_IDT(
            (&*IDT_TABLE as *const IdtTable) as u32,
            core::mem::size_of_val(&*IDT_TABLE) as u16,
        )
    }
}
