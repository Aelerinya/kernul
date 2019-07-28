use core::fmt::Error as FmtError;
use core::fmt::Write;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_ADDRESS: usize = 0xb8000;
const DEFAULT_STYLE: u8 = 0x07;

struct Position {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
struct VgaCharacter {
    codepoint: u8,
    style: u8,
}

impl VgaCharacter {
    const EMPTY: VgaCharacter = VgaCharacter {
        codepoint: ' ' as u8,
        style: DEFAULT_STYLE,
    };
}

type VgaBuffer = [[VgaCharacter; BUFFER_WIDTH]; BUFFER_HEIGHT];

/// Object that manages the VgaBuffer as a common terminal
pub struct VgaScreen {
    cursor: Position,
    buffer: &'static mut VgaBuffer,
}

impl VgaScreen {
    pub fn new() -> VgaScreen {
        VgaScreen {
            cursor: Position { x: 0, y: 0 },
            buffer: unsafe { &mut *(BUFFER_ADDRESS as *mut VgaBuffer) },
        }
    }

    fn print_char(&mut self, c: char) {
        match c {
            // Write character and move cursor forward
            ' '..='~' => {
                self.buffer[self.cursor.y][self.cursor.x] = VgaCharacter {
                    codepoint: c as u8,
                    style: DEFAULT_STYLE,
                };
                self.cursor.x += 1;
                if self.cursor.x >= BUFFER_WIDTH {
                    self.cursor.x = 0;
                    self.cursor.y += 1;
                    if self.cursor.y >= BUFFER_HEIGHT {
                        self.scroll();
                        self.cursor.y = BUFFER_HEIGHT;
                    }
                }
            }
            // \n: Move cursor to the start of the next line
            '\n' => {
                self.cursor.x = 0;
                self.cursor.y += 1;
                if self.cursor.y >= BUFFER_HEIGHT {
                    self.scroll();
                    self.cursor.y = BUFFER_HEIGHT - 1;
                }
            }
            // \b: Move cursor back and delete character
            '\x08' => {
                if self.cursor.x != 0 || self.cursor.y != 0 {
                    if self.cursor.x == 0 {
                        self.cursor.x = BUFFER_WIDTH - 1;
                        self.cursor.y -= 1;
                    } else {
                        self.cursor.x -= 1;
                    }
                    self.buffer[self.cursor.y][self.cursor.x].codepoint = ' ' as u8;
                }
            }
            // \t: Align the cursor on 4 character columns
            '\t' => {
                let mut next_stop = (self.cursor.x / 4 + 1) * 4;
                if next_stop >= BUFFER_WIDTH {
                    next_stop = BUFFER_WIDTH - 1;
                }
                for x in self.cursor.x..next_stop {
                    self.buffer[self.cursor.y][x].codepoint = ' ' as u8;
                }
                self.cursor.x = next_stop;
            }

            // \r: Move the cursor back to column 0 on the same line
            '\r' => {
                self.cursor.x = 0;
            }
            _ => {}
        }
    }

    // Scroll the screen: move every line up, and empty the last line
    fn scroll(&mut self) {
        for line in 0..(BUFFER_HEIGHT - 1) {
            for column in 0..BUFFER_WIDTH {
                self.buffer[line][column] = self.buffer[line + 1][column];
            }
        }
        for c in self.buffer[BUFFER_HEIGHT - 1].iter_mut() {
            *c = VgaCharacter::EMPTY;
        }
    }
}

impl Write for VgaScreen {
    fn write_str(&mut self, s: &str) -> Result<(), FmtError> {
        for c in s.chars() {
            self.print_char(c);
        }
        Ok(())
    }
}
