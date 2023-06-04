type Rgba = [u8; 4];

fn blend_rgba(bg: Rgba, fg: Rgba) -> Rgba {
    // convert all our values to floats in the range [0, 1]
    let r_fg: f32 = fg[0] as f32 / 255.0;
    let g_fg: f32 = fg[1] as f32 / 255.0;
    let b_fg: f32 = fg[2] as f32 / 255.0;
    let a_fg: f32 = fg[3] as f32 / 255.0;
    let r_bg: f32 = bg[0] as f32 / 255.0;
    let g_bg: f32 = bg[1] as f32 / 255.0;
    let b_bg: f32 = bg[2] as f32 / 255.0;
    let a_bg: f32 = bg[3] as f32 / 255.0;

    // calculate final alpha
    let a = a_fg + a_bg * (1.0 - a_fg);

    // calculate final colour channels
    let r = ((r_fg * a_fg) + (r_bg * a_bg * (1.0 - a_fg))) / a;
    let g = ((g_fg * a_fg) + (g_bg * a_bg * (1.0 - a_fg))) / a;
    let b = ((b_fg * a_fg) + (b_bg * a_bg * (1.0 - a_fg))) / a;

    // reconvert our floats to be ints in the range [0, 255]
    [
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    ]
}

#[test]
fn test_blend_rgba() {
    // test that layering translucent blue above opaque red results in opaque purple.
    let opaque_red = [255, 0, 0, 255];
    let translucent_blue = [0, 0, 255, 127];
    let opaque_purple = [127, 0, 127, 255];
    assert_eq!(blend_rgba(opaque_red, translucent_blue), opaque_purple);
}
