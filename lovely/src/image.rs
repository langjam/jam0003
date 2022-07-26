use raqote::*;

const COLORS: [&str; 256] = [
    "#c4b9b8", "#879f84", "#a6814c", "#ff9966", "#aa1155", "#7a81ff", "#8e473b", "#f3dfb6",
    "#dfac4c", "#4b3621", "#feff32", "#a57c5b", "#52b4ca", "#fffc79", "#879877", "#bb1133",
    "#eee9d9", "#52b4d3", "#ffff81", "#4f4554", "#c68f65", "#d2e7ca", "#0000ff", "#2ee8bb",
    "#eebb88", "#eddd59", "#7f5f00", "#eeaa11", "#35fa00", "#ffefd6", "#bb11aa", "#dab4cc",
    "#fff0db", "#987d73", "#c4fff7", "#eae0c8", "#e2c779", "#ff4d00", "#334d41", "#f83800",
    "#ddedbd", "#ffdd44", "#efd7ab", "#66eeee", "#aa3333", "#006663", "#ffb75f", "#c6bb9c",
    "#685c53", "#fff1dc", "#313390", "#ff6f01", "#8f8ce7", "#cf0234", "#ff9966", "#5f5537",
    "#342931", "#f6eeed", "#fe019a", "#772299", "#ff6ec7", "#f2e6e1", "#ffb49b", "#252525",
    "#dcf1c7", "#674876", "#eeaa55", "#c7031e", "#7a2e4d", "#9f0000", "#eed683", "#e6dee6",
    "#fdd7e4", "#95859c", "#bfa58a", "#16141c", "#7b9a6d", "#cc6666", "#ddece0", "#e398af",
    "#004488", "#fea993", "#6d5698", "#ef1de7", "#ff028d", "#eae9e7", "#f6e2ea", "#b8b8f8",
    "#ccddcc", "#b06500", "#ecebe5", "#7fbb9e", "#88b5c4", "#93c572", "#dddd88", "#805b87",
    "#698890", "#ebe2cf", "#ee2222", "#c95a49", "#220011", "#f1ebc8", "#006400", "#553b39",
    "#ffa177", "#cbc5c6", "#738f5d", "#32575d", "#fed55d", "#a2bffe", "#98333a", "#d7e7d0",
    "#958b84", "#ee0000", "#3b2b2c", "#ff8656", "#f0fff0", "#44232f", "#faebd7", "#786e38",
    "#080813", "#5b6f55", "#99c5c4", "#332e2e", "#7b4d3a", "#536267", "#eff0d3", "#f4d493",
    "#fcd917", "#e16233", "#f8e0e7", "#e59b34", "#ebe5d0", "#393540", "#e4f3e0", "#d6d7d2",
    "#c19a13", "#d29380", "#babfbc", "#3b638c", "#ffe29b", "#99eeff", "#eebb33", "#4a3b6a",
    "#c6bbdb", "#ab6f60", "#ff9b87", "#b08f42", "#a67283", "#b1832f", "#fedbb7", "#fcd7ba",
    "#ee3366", "#89a203", "#484a46", "#f878f8", "#b66325", "#c1f80a", "#6dbac0", "#4b373a",
    "#03012d", "#ff2600", "#555570", "#ff7a00", "#8edacc", "#ffcc77", "#cfac47", "#a15325",
    "#ff4466", "#f9f1dd", "#816d5e", "#5f6957", "#f3e0d8", "#006380", "#c26157", "#f9e3b4",
    "#c88ca4", "#b87333", "#ea9073", "#ee1133", "#e4d9c5", "#dbe7e3", "#ff0e0e", "#eebe1b",
    "#d8caa9", "#9bc2b1", "#b0003c", "#bae5d6", "#e38fac", "#f6cbca", "#ee4433", "#73383c",
    "#3f4250", "#000066", "#ffff33", "#bda58b", "#e0b0ff", "#acddaf", "#a9afaa", "#cee1f2",
    "#7ad7ad", "#e5dae1", "#906a54", "#938b4b", "#86c4da", "#a17a83", "#008e80", "#ebebeb",
    "#883377", "#9a0eea", "#11cc55", "#eecc44", "#aaff32", "#e2c9ce", "#f2ab46", "#fd8f79",
    "#92898a", "#787489", "#a6ab9b", "#cef0cc", "#443388", "#fedc57", "#ed4b00", "#a58459",
    "#553311", "#ffb07c", "#d4ffff", "#534b4f", "#74857f", "#007fff", "#332266", "#bc6f37",
    "#aaaa77", "#cce2f3", "#fff8dc", "#5c5d5d", "#f3f4d9", "#c7bba4", "#cf758a", "#fbec5d",
    "#fffbf8", "#b7d2e3", "#eeee66", "#f6efe1", "#ffe39b", "#4e312d", "#e75480", "#bbb3a2",
    "#3c2f23", "#8f7f85", "#e3cdc2", "#f3efcd", "#152eff", "#d9eae5", "#dabe82", "#ce2029",
];

pub fn draw(bytes: &[u8]) -> DrawTarget {
    let tile_width = (bytes.len() as f32).sqrt().floor() + 1.0;
    let size = tile_width.powi(2);

    let mut dt = DrawTarget::new(size as i32, size as i32);

    let mut current_x = 0.0f32;
    let mut current_y = 0.0f32;
    for byte in bytes {
        dt.fill_rect(
            current_x,
            current_y,
            tile_width,
            tile_width,
            &Source::Solid(SolidSource {
                r: u8::from_str_radix(&COLORS[*byte as usize][1..3], 16).unwrap(),
                g: u8::from_str_radix(&COLORS[*byte as usize][3..5], 16).unwrap(),
                b: u8::from_str_radix(&COLORS[*byte as usize][5..7], 16).unwrap(),
                a: 0xFF,
            }),
            &DrawOptions::new(),
        );

        if current_x + tile_width == size {
            current_x = 0.0;
            current_y += tile_width;
        } else {
            current_x += tile_width;
        }
    }

    let num_tiles_left = size - (bytes.len() as f32);
    for _ in 0..(num_tiles_left as usize) {
        dt.fill_rect(
            current_x,
            current_y,
            tile_width,
            tile_width,
            &Source::Solid(SolidSource {
                r: u8::from_str_radix(&COLORS[0][1..3], 16).unwrap(),
                g: u8::from_str_radix(&COLORS[0][3..5], 16).unwrap(),
                b: u8::from_str_radix(&COLORS[0][5..7], 16).unwrap(),
                a: 0xFF,
            }),
            &DrawOptions::new(),
        );

        if current_x + tile_width == size {
            current_x = 0.0;
            current_y += tile_width;
        } else {
            current_x += tile_width;
        }
    }

    dt
}

pub fn decode(image_bytes: &[u8], size: usize) -> Vec<u8> {
    let mut result = Vec::new();
    let tile_width = (size as f32).sqrt() as usize;

    let pixels = image_bytes.chunks(4).collect::<Vec<_>>();

    let mut cursor = 0;
    for _ in 0..tile_width {
        for _ in 0..tile_width {
            let r = pixels[cursor][0];
            let g = pixels[cursor][1];
            let b = pixels[cursor][2];
            let color_string = format!("#{:02x}{:02x}{:02x}", r, g, b);

            let idx = COLORS.iter().position(|el| *el == color_string);
            result.push(idx.unwrap() as u8);

            cursor += tile_width;
        }
        cursor += size * (tile_width - 1);
    }

    result
}
