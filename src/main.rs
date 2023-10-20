use std::fs::read;
use raylib::prelude::*;

fn vec_u8_to_u16(vec: Vec<u8>) -> Vec<u16> {
    let mut res = vec![];
    
    for i in 0..vec.len()/2 {
        res.push(vec[i + 1] as u16 | (vec[i] as u16) << 8)
    }

    res
}

fn compute_freqs(data: Vec<u16>) -> Vec<f64> {
    if data.len() == 0 { return vec![] }

    let mut counts = vec![0_u32; 65536];
    let mut max = 0;

    for w in data {
        counts[w as usize] += 1;
        if counts[w as usize] > max { max += 1 }
    }

    let mut freqs = vec![];

    for c in counts {
        freqs.push(c as f64 / max as f64)
    }

    freqs
}

fn update_canvas(canvas: &mut Image, freqs: &Vec<f64>, bright: f64, cont: f64) {
    if freqs.len() == 0 { return }

    canvas.clear_background(Color::BLACK);

    for (i, p) in freqs.iter().enumerate() {
        let b = ((2.0_f64.powf(bright) * (*p).powf(cont))
            .clamp(0., 1.) * 255.).floor() as u8;

        canvas.draw_pixel(
            (i % 256) as i32,
            (i / 256) as i32,
            Color{r: b, g: b, b: b, a: 255}
        )
    }
}

fn main() {
    raylib::set_trace_log(TraceLogLevel::LOG_WARNING);
    
    let (mut rl, thread) = raylib::init()
        .size(768, 824)
        .title("filevis")
        .build();

    rl.set_target_fps(60);

    let mut bright = 0.0_f64;
    let mut cont = 1.0_f64;
    let mut canvas = Image::gen_image_color(256, 256, Color::BLACK);
    let mut data: Vec<u16>;
    let mut freqs: Vec<f64> = vec![];

    while !rl.window_should_close() {
        let key = rl.get_key_pressed();

        if key == Some(KeyboardKey::KEY_UP) {
            bright += 0.5;
            update_canvas(&mut canvas, &freqs, bright, cont)
        } else if key == Some(KeyboardKey::KEY_DOWN) {
            bright = (bright - 0.5).max(0.);
            update_canvas(&mut canvas, &freqs, bright, cont)
        } else if key == Some(KeyboardKey::KEY_LEFT) {
            cont = (cont - 0.125).max(0.);
            update_canvas(&mut canvas, &freqs, bright, cont)
        } else if key == Some(KeyboardKey::KEY_RIGHT) {
            cont += 0.125;
            update_canvas(&mut canvas, &freqs, bright, cont)
        } else if key == Some(KeyboardKey::KEY_SPACE) {
            bright = 0.;
            cont = 1.;
            update_canvas(&mut canvas, &freqs, bright, cont)
        }

        let files = rl.get_dropped_files();
        
        if files.len() != 0 {
            let filename = files[0].to_string();
            data = vec_u8_to_u16(read(filename).unwrap_or(vec![]));
            freqs = compute_freqs(data);
            bright = 0.;
            cont = 1.;
            update_canvas(&mut canvas, &freqs, bright, cont);
            rl.clear_dropped_files()
        }

        let canvas_tex = rl.load_texture_from_image(&thread, &canvas).unwrap();

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_texture_ex(&canvas_tex, Vector2{x: 0., y: 0.}, 0., 3., Color::WHITE);
        d.draw_text(&format!("bright: +{}", bright), 4, 772, 20, Color::WHITE);
        d.draw_text(&format!("cont: *{}", cont), 4, 800, 20, Color::WHITE)
    }
}