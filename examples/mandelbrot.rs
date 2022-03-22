const WIDTH: usize = 128;
const HEIGHT: usize = 96;

fn main() {
    let mut framebuffer = vec![false; WIDTH * HEIGHT];
    mandelbrot(&mut framebuffer);
    let f = braillefb::Framebuffer::new(&framebuffer, WIDTH, HEIGHT);
    print!("{}", f.into_iter().collect::<String>());
}

pub fn mandelbrot(framebuffer: &mut [bool]) {
    let max_iterations = 50;
    let range = (-2.0, 2.0);
    let min_im = -1.2;
    let max_im = min_im + (range.1 - range.0) * HEIGHT as f64 / WIDTH as f64;
    let re_factor = (range.1 - range.0) / (WIDTH - 1) as f64;
    let im_factor = (max_im - min_im) / (HEIGHT - 1) as f64;

    for y in 0..HEIGHT {
        let c_im = max_im - y as f64 * im_factor;

        for x in 0..WIDTH {
            let c_re = range.0 + x as f64 * re_factor;
            let mut z_re = c_re;
            let mut z_im = c_im;

            let mut out = 0;
            for iterations in 0..max_iterations {
                let z_re2 = z_re * z_re;
                let z_im2 = z_im * z_im;

                if iterations != max_iterations {
                    out = iterations;
                } else {
                    out = 0;
                }

                if z_re2 + z_im2 > 4. {
                    break;
                }

                z_im = 2. * z_re * z_im + c_im;
                z_re = z_re2 - z_im2 + c_re;
            }

            framebuffer[x + y * WIDTH] = out > 45;
        }
    }
}
