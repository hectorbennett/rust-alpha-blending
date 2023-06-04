use std::simd::u32x2;

type Rgba = [u8; 4];

#[inline]
pub fn fast_divide_by_255(i: u32) -> u32 {
    (i + 1 + (i >> 8)) >> 8
}

#[test]
fn test_fast_divide_by_255() {
    assert_eq!(fast_divide_by_255(33654), 131);
}

pub fn blend_rgba(bg: Rgba, fg: Rgba) -> Rgba {
    let r_fg = fg[0] as u32;
    let g_fg = fg[1] as u32;
    let b_fg = fg[2] as u32;
    let a_fg = fg[3] as u32;
    let r_bg = bg[0] as u32;
    let g_bg = bg[1] as u32;
    let b_bg = bg[2] as u32;
    let a_bg = bg[3] as u32;

    let thing_1 = 255 * a_fg;
    let thing_2 = 255 * a_bg - a_fg * a_bg;

    // calculate final alpha * 255
    let a_0 = thing_1 + thing_2;

    // calculate red and green together with simd
    let rg = (u32x2::splat(thing_1) * u32x2::from([r_fg, g_fg])
        + u32x2::splat(thing_2) * u32x2::from([r_bg, g_bg]))
        / u32x2::splat(a_0);

    // calculate blue on its own
    let b = (thing_1 * b_fg + thing_2 * b_bg) / a_0;

    [
        rg[0] as u8,
        rg[1] as u8,
        b as u8,
        fast_divide_by_255(a_0) as u8,
    ]
}

#[test]
fn test_blend_rgba() {
    // test that layering translucent blue above opaque red results in opaque purple.
    let opaque_red = [255, 0, 0, 255];
    let translucent_blue = [0, 0, 255, 127];
    let opaque_purple = [128, 0, 127, 255];
    assert_eq!(blend_rgba(opaque_red, translucent_blue), opaque_purple);

    assert_eq!(
        blend_rgba([10, 20, 30, 40], [50, 60, 70, 80]),
        [39, 49, 59, 107]
    );
}
