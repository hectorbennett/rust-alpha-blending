type Rgba = [u8; 4];

pub fn blend_rgba(bg: Rgba, fg: Rgba) -> Rgba {
    // convert all our values to floats in the range [0, 1]
    let mut r_fg: f32 = fg[0] as f32 / 255.0;
    let mut g_fg: f32 = fg[1] as f32 / 255.0;
    let mut b_fg: f32 = fg[2] as f32 / 255.0;
    let a_fg: f32 = fg[3] as f32 / 255.0;
    let mut r_bg: f32 = bg[0] as f32 / 255.0;
    let mut g_bg: f32 = bg[1] as f32 / 255.0;
    let b_bg: f32 = bg[2] as f32 / 255.0;
    let mut a_bg: f32 = bg[3] as f32 / 255.0;

    // calculate final alpha
    let a = a_fg + a_bg * (1.0 - a_fg);

    let gamma = 2.2;

    // gamma expand the rgb values
    r_fg = r_fg.powf(gamma);
    g_fg = g_fg.powf(gamma);
    b_fg = b_fg.powf(gamma);
    r_bg = r_bg.powf(gamma);
    g_bg = g_bg.powf(gamma);
    a_bg = a_bg.powf(gamma);

    // calculate final colour channels
    let mut r = ((r_fg * a_fg) + (r_bg * a_bg * (1.0 - a_fg))) / a;
    let mut g = ((g_fg * a_fg) + (g_bg * a_bg * (1.0 - a_fg))) / a;
    let mut b = ((b_fg * a_fg) + (b_bg * a_bg * (1.0 - a_fg))) / a;

    // gamma compress the final colour channels
    r = r.powf(1.0 / gamma);
    g = g.powf(1.0 / gamma);
    b = b.powf(1.0 / gamma);

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
    let opaque_purple = [186, 0, 185, 255];
    assert_eq!(blend_rgba(opaque_red, translucent_blue), opaque_purple);

    assert_eq!(
        blend_rgba([10, 20, 30, 40], [50, 60, 70, 80]),
        [43, 52, 63, 107]
    );
}
