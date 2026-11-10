fn main() {
    for y in -21..21 {
        for x in -75..75 {
            let a: f32 = x as f32 / 25.0;
            let b: f32 = y as f32 / 11.0;
            let r = (a.powf(2.0) + b.powf(2.0)).sqrt();
            let v = b / 0.16;
            let d: f32 = (a.powf(2.0) + v.powf(2.0)).sqrt();
            let mut light: f32 = 0.0;

            if 1.3 < d && d < 3.4 && (r > 1.0 || b > 0.0) {
                light = (3.4 - d) / 2.1 * (1.0 - 0.3 * a / d);
            } else if 1.0 < r && r < 1.5 {
                light = (1.5 - r) * 2.0;
            } else if r >= 1.5 && (x * 7 + y * 13 as i32).rem_euclid(89) == 0 {
                light = 0.5;
            }
            let q: i32 = (light.clamp(0.0, 0.99) * 256.0) as i32;
            let i = q * 10 / 256;
            let c = format!("\x1b[38;2;{};{};{}m", q, q * q / 330, q * q * q / 110000);
            print!("{}{}", c, b" .:-=+*#%@"[i as usize] as char);
        }
        println!("");
    }
}
