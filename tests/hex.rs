use arch_pkg_text::value::{Hex128, Hex256, SkipOrArray, SkipOrHex128, SkipOrHex256};
use pretty_assertions::assert_eq;

#[test]
fn hex128() {
    let hex128 = Hex128("165F04122017EC76579594B17F15F8EB");
    assert_eq!(
        hex128.u8_array(),
        Some([
            0x16, 0x5f, 0x04, 0x12, 0x20, 0x17, 0xec, 0x76, 0x57, 0x95, 0x94, 0xb1, 0x7f, 0x15,
            0xf8, 0xeb,
        ]),
    );
    assert_eq!(hex128.u128(), Some(0x165f04122017ec76579594b17f15f8eb));
}

#[test]
fn hex256() {
    let hex256 = Hex256("37CBA20B05B899DCBE2E565B1C20B7CFC6411FFCB014B977EDB2D8AFBC3A530B");
    assert_eq!(
        hex256.u8_array(),
        Some([
            0x37, 0xcb, 0xa2, 0x0b, 0x05, 0xb8, 0x99, 0xdc, 0xbe, 0x2e, 0x56, 0x5b, 0x1c, 0x20,
            0xb7, 0xcf, 0xc6, 0x41, 0x1f, 0xfc, 0xb0, 0x14, 0xb9, 0x77, 0xed, 0xb2, 0xd8, 0xaf,
            0xbc, 0x3a, 0x53, 0x0b,
        ]),
    );
}

#[test]
fn skip_or_hex128() {
    let hex128 = SkipOrHex128("SKIP");
    assert_eq!(hex128.u8_array(), Some(SkipOrArray::Skip));
    assert_eq!(hex128.u128(), Some(None));

    let hex128 = SkipOrHex128("165f04122017ec76579594b17f15f8eb");
    assert_eq!(
        hex128.u8_array(),
        Some(SkipOrArray::Array([
            0x16, 0x5f, 0x04, 0x12, 0x20, 0x17, 0xec, 0x76, 0x57, 0x95, 0x94, 0xb1, 0x7f, 0x15,
            0xf8, 0xeb,
        ])),
    );
    assert_eq!(
        hex128.u128(),
        Some(Some(0x165f04122017ec76579594b17f15f8eb)),
    );
}

#[test]
fn skip_or_hex256() {
    let hex256 = SkipOrHex256("SKIP");
    assert_eq!(hex256.u8_array(), Some(SkipOrArray::Skip));

    let hex256 = SkipOrHex256("37cba20b05b899dcbe2e565b1c20b7cfc6411ffcb014b977edb2d8afbc3a530b");
    assert_eq!(
        hex256.u8_array(),
        Some(SkipOrArray::Array([
            0x37, 0xcb, 0xa2, 0x0b, 0x05, 0xb8, 0x99, 0xdc, 0xbe, 0x2e, 0x56, 0x5b, 0x1c, 0x20,
            0xb7, 0xcf, 0xc6, 0x41, 0x1f, 0xfc, 0xb0, 0x14, 0xb9, 0x77, 0xed, 0xb2, 0xd8, 0xaf,
            0xbc, 0x3a, 0x53, 0x0b,
        ])),
    );
}
