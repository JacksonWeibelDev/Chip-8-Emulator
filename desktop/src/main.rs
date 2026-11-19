use chip8_core::{Emu, SCREEN_HEIGHT, SCREEN_WIDTH};
use raylib::prelude::*;
use std::{env, error::Error, fs, io};

const SCALE: i32 = 15;
const TICKS_PER_FRAME: usize = 10;
// A brief phosphor-like afterglow keeps XOR-erased sprites visible between redraws.
const PIXEL_PERSISTENCE_FRAMES: u8 = 3;
const KEY_MAPPING: [(KeyboardKey, usize); 16] = [
    (KeyboardKey::KEY_ONE, 0x1),
    (KeyboardKey::KEY_TWO, 0x2),
    (KeyboardKey::KEY_THREE, 0x3),
    (KeyboardKey::KEY_FOUR, 0xC),
    (KeyboardKey::KEY_Q, 0x4),
    (KeyboardKey::KEY_W, 0x5),
    (KeyboardKey::KEY_E, 0x6),
    (KeyboardKey::KEY_R, 0xD),
    (KeyboardKey::KEY_A, 0x7),
    (KeyboardKey::KEY_S, 0x8),
    (KeyboardKey::KEY_D, 0x9),
    (KeyboardKey::KEY_F, 0xE),
    (KeyboardKey::KEY_Z, 0xA),
    (KeyboardKey::KEY_X, 0x0),
    (KeyboardKey::KEY_C, 0xB),
    (KeyboardKey::KEY_V, 0xF),
];

struct DisplayPersistence {
    remaining: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl DisplayPersistence {
    fn new() -> Self {
        Self {
            remaining: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    fn begin_frame(&mut self) {
        for remaining in &mut self.remaining {
            *remaining = remaining.saturating_sub(1);
        }
    }

    fn observe(&mut self, display: &[bool]) {
        for (remaining, &pixel) in self.remaining.iter_mut().zip(display) {
            if pixel {
                *remaining = PIXEL_PERSISTENCE_FRAMES;
            }
        }
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os();
    let program = args.next().unwrap_or_default();
    let rom_path = args.next().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Usage: {} <rom-path>", program.to_string_lossy()),
        )
    })?;
    if args.next().is_some() {
        return Err(
            io::Error::new(io::ErrorKind::InvalidInput, "Expected exactly one ROM path").into(),
        );
    }
    let rom = fs::read(&rom_path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("Could not read ROM {}: {error}", rom_path.to_string_lossy()),
        )
    })?;
    let mut emu = Emu::new();
    emu.load_rom(&rom)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

    let (mut raylib, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32 * SCALE, SCREEN_HEIGHT as i32 * SCALE)
        .title("CHIP-8 Emulator")
        .build();
    let mut display = DisplayPersistence::new();
    raylib.set_target_fps(60);
    while !raylib.window_should_close() {
        for &(key, chip8_key) in &KEY_MAPPING {
            emu.keypress(chip8_key, raylib.is_key_down(key));
        }
        display.begin_frame();
        for _ in 0..TICKS_PER_FRAME {
            emu.tick();
            display.observe(emu.get_display());
        }
        emu.tick_timers();

        let mut drawing = raylib.begin_drawing(&thread);
        drawing.clear_background(Color::BLACK);
        for (index, &remaining) in display.remaining.iter().enumerate() {
            if remaining > 0 {
                let x = (index % SCREEN_WIDTH) as i32 * SCALE;
                let y = (index / SCREEN_WIDTH) as i32 * SCALE;
                drawing.draw_rectangle(x, y, SCALE, SCALE, Color::WHITE);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_short_lived_pixels_and_fades_them() {
        let mut persistence = DisplayPersistence::new();
        let mut pixels = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        persistence.begin_frame();
        pixels[42] = true;
        persistence.observe(&pixels);
        pixels[42] = false;
        persistence.observe(&pixels);
        assert!(persistence.remaining[42] > 0);
        for _ in 1..PIXEL_PERSISTENCE_FRAMES {
            persistence.begin_frame();
            persistence.observe(&pixels);
            assert!(persistence.remaining[42] > 0);
        }
        persistence.begin_frame();
        assert_eq!(persistence.remaining[42], 0);
    }
}
