pub enum ObjectPalette {
    Palette0,
    Palette1,
}

/// Unpack palette from single byte to intensity values (0-3) for each of the four possible pixel
/// values.
///
/// From the Pandocs:
///
/// > - Bit 7-6 - Data for Dot Data `11` (Normally darkest color)
/// > - Bit 5-4 - Data for Dot Data `10`
/// > - Bit 3-2 - Data for Dot Data `01`
/// > - Bit 1-0 - Data for Dot Data `00` (Normally lightest color)
/// >
/// > This selects the shade of grays to use for the background (BG) & window pixels. Since each
/// > pixel uses 2 bits, the corresponding shade will be selected from here.
pub fn unpack_palette(palette: u8) -> [u8; 4] {
    let mut unpacked_palette = [0u8; 4];

    unpacked_palette
        .iter_mut()
        .enumerate()
        .for_each(|(idx, palette_entry)| {
            *palette_entry = (palette >> (idx * 2)) & 3;
        });

    unpacked_palette
}

#[test]
fn test_unpack_palette() {
    assert_eq!([0, 0, 0, 0], unpack_palette(0b0000_0000));

    assert_eq!([1, 0, 0, 0], unpack_palette(0b0000_0001));
    assert_eq!([0, 1, 0, 0], unpack_palette(0b0000_0100));
    assert_eq!([0, 0, 1, 0], unpack_palette(0b0001_0000));
    assert_eq!([0, 0, 0, 1], unpack_palette(0b0100_0000));

    assert_eq!([2, 0, 0, 0], unpack_palette(0b0000_0010));
    assert_eq!([0, 2, 0, 0], unpack_palette(0b0000_1000));
    assert_eq!([0, 0, 2, 0], unpack_palette(0b0010_0000));
    assert_eq!([0, 0, 0, 2], unpack_palette(0b1000_0000));

    assert_eq!([3, 0, 0, 0], unpack_palette(0b0000_0011));
    assert_eq!([0, 3, 0, 0], unpack_palette(0b0000_1100));
    assert_eq!([0, 0, 3, 0], unpack_palette(0b0011_0000));
    assert_eq!([0, 0, 0, 3], unpack_palette(0b1100_0000));

    assert_eq!([3, 3, 3, 3], unpack_palette(0b1111_1111));
}
