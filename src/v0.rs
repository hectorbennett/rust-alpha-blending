type Rgba = [u8; 4];

pub fn blend_rgba(_bg: Rgba, _fg: Rgba) -> Rgba {
    unimplemented!();
}

#[test]
fn test_blend_rgba() {
    // test that layering translucent blue above opaque red results in opaque purple.
    let opaque_red = [255, 0, 0, 255];
    let translucent_blue = [0, 0, 255, 128];
    let opaque_purple = [188, 0, 188, 255];
    assert_eq!(blend_rgba(opaque_red, translucent_blue), opaque_purple);
}
