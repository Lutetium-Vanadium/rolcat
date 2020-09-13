pub fn print(s: &str, h: f32, shift: f32) {
    let mut h = h;
    for chr in s.split("") {
        let (r, g, b) = get_rgb(h);
        print!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, chr);
        h = (360.0 + h + shift) % 360.0;
    }
    println!();
}

fn get_rgb(h: f32) -> (u8, u8, u8) {
    let hv = h / 60.;
    let hi = hv.floor() % 6.;
    let f = (hv - hi) * 255.0;
    let q = 255.0 - f;

    if hi == 0. {
        (255, f as u8, 0)
    } else if hi == 1. {
        (q as u8, 255, 0)
    } else if hi == 2. {
        (0, 255, f as u8)
    } else if hi == 3. {
        (0, q as u8, 255)
    } else if hi == 4. {
        (f as u8, 0, 255)
    } else if hi == 5. {
        (255, 0, q as u8)
    } else {
        println!("---- UNREACHABLE ----");
        println!("h: {}, hv: {}, hi: {}", h, hv, hi);
        unreachable!()
    }
}
