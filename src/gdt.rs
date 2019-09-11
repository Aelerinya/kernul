use bitfield::bitfield;
use bitflags::bitflags;
use lazy_static::lazy_static;

bitflags! {
    /// Bit flags describing the access permissions of a data segment
    struct GdtAccess: u8 {
        const PRESENT = 0b10000000;
        const USER_RING = 0b01100000;
        const CODE_SEGMENT = 0b00011000;
        const DATA_SEGMENT = 0b00010000;
        const DIRECTION_DOWN = 0b100;
        const CONFORMING = 0b100;
        const CODE_READABLE = 0b10;
        const DATA_WRITABLE = 0b10;
        const ACCESSED = 0b1;
    }
}

impl From<u8> for GdtAccess {
    fn from(value: u8) -> GdtAccess {
        GdtAccess::from_bits(value).expect("Invalid GDT access flags")
    }
}

bitflags! {
    /// Various flags of a data segment: granularity and size
    struct GdtFlags: u8 {
        const PAGE_GRANULARITY = 0b1000;
        const PROTECTED_MODE = 0b0100;
        const LONG_MODE = 0b0010;
    }
}
impl From<u8> for GdtFlags {
    fn from(value: u8) -> GdtFlags {
        GdtFlags::from_bits(value).expect("Invalid GDT flags")
    }
}

bitfield! {
    /// Structure describing a entry in the Global Descriptor Table (GDT)
    /// It contains the base of the segment, its size (limit), its access permissions and flags
    #[derive(Clone, Copy)]
    #[repr(transparent)]
    struct GdtEntry(u64);
    impl Debug;
    u16, limit_low, set_limit_low: 15, 0;
    u16, base_low, set_base_low: 31, 16;
    u8, base_middle, set_base_middle: 39, 32;
    u8, into GdtAccess, access, set_access: 47, 40;
    u8, limit_high, set_limit_high: 51, 48;
    u8, into GdtFlags, flags, set_flags: 55, 52;
    u8, base_high, set_base_high: 63, 56;
}

/// Error returned when a GDT entry is malformed
#[derive(Debug)]
enum GdtError {
    InvalidLimit,
}

impl GdtEntry {
    /// Creates a new GDT entry
    fn new(
        base: u64,
        limit: u32,
        mut access: GdtAccess,
        flags: GdtFlags,
    ) -> Result<GdtEntry, GdtError> {
        let mut entry = GdtEntry(0);
        entry.set_base(base);
        entry.set_limit(limit)?;
        // Set the present flag
        access |= GdtAccess::PRESENT;
        entry.set_access(access.bits());
        entry.set_flags(flags.bits());
        Ok(entry)
    }

    fn set_base(&mut self, base: u64) {
        self.set_base_low((base & 0xFFFF) as u16);
        self.set_base_middle((base >> 16 & 0xFF) as u8);
        self.set_base_high((base >> 24 & 0xFF) as u8);
    }

    fn set_limit(&mut self, limit: u32) -> Result<(), GdtError> {
        if limit > 0xFFFFF {
            Err(GdtError::InvalidLimit)
        } else {
            self.set_limit_low((limit & 0xFFFF) as u16);
            self.set_limit_high((limit >> 16 & 0xFF) as u8);
            Ok(())
        }
    }
}

const GDT_ENTRIES_COUNT: usize = 4;
lazy_static! {
    #[derive(Debug)]
    static ref GDT_TABLE: [GdtEntry; GDT_ENTRIES_COUNT] = [
        // Null entry
        GdtEntry::new(0, 0, GdtAccess::empty(), GdtFlags::PROTECTED_MODE).unwrap(),
        // Code segment
        GdtEntry::new(0, 0xFFFFF,
            GdtAccess::CODE_SEGMENT | GdtAccess::CODE_READABLE,
            GdtFlags::PROTECTED_MODE | GdtFlags::PAGE_GRANULARITY).unwrap(),
        // Data segment
        GdtEntry::new(0, 0xFFFFF,
            GdtAccess::DATA_SEGMENT | GdtAccess::DATA_WRITABLE,
            GdtFlags::PROTECTED_MODE | GdtFlags::PAGE_GRANULARITY).unwrap(),
        // TSS segment
        GdtEntry::new(0, 0, GdtAccess::ACCESSED, GdtFlags::PROTECTED_MODE).unwrap()
    ];
}

// Assembly routine loading the GDT table
extern "C" {
    fn load_GDT(offset: u32, limit: u16);
}

/// Initializes the GDT table and loads it into memory
pub fn init_gdt() {
    unsafe {
        load_GDT(
            (&*GDT_TABLE as *const [GdtEntry; GDT_ENTRIES_COUNT]) as u32,
            core::mem::size_of_val(&*GDT_TABLE) as u16,
        )
    }
}
