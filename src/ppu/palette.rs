pub enum ObjectPalette {
    Palette0,
    Palette1,
}

pub fn unpack_palette(palette: u8) -> [u8; 4] {
    let mut unpacked_palette = [0u8; 4];
    for i in 0..4 {
        unpacked_palette[i] = (palette >> (i * 2)) & 3;
    }
    unpacked_palette
}
