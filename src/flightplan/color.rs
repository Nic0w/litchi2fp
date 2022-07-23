pub const POI_COLORS: &[i32] = &[
    -7589836, -7581644, -6537400, -4699067, -4631765, -3316660, -2600147, -1022921, -51356, -49820,
    -47804,
];

#[cfg(test)]
mod tests {

    use palette::rgb::channels::Argb;
    use palette::Hsla;
    use palette::IntoColor;
    use palette::Srgba;

    #[test]
    fn colors() {
        for c in super::POI_COLORS {
            let srgba = Srgba::from_u32::<Argb>(*c as u32);

            let srgba_float = srgba.into_format::<f32, f32>();

            let hsla: Hsla = srgba_float.into_color();

            println!("0x{:x} {:?}\n{:?}\n", c, srgba, hsla);
        }
    }
}
