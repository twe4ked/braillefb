const WIDTH: usize = 96;
const HEIGHT: usize = 96 - 16;

fn main() {
    let mut framebuffer = vec![false; WIDTH * HEIGHT];
    mandelbrot(&mut framebuffer);
    print!(
        "{}",
        braillefb::Framebuffer::new(&framebuffer, WIDTH, HEIGHT)
    );
}

// https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set#Escape_time_algorithm
fn mandelbrot(framebuffer: &mut [bool]) {
    let max = 64;
    for py in 0..HEIGHT {
        for px in 0..WIDTH {
            let x0 = (px as f32 * 2.47 / WIDTH as f32) - 2.0;
            // +8 here to account for the -16 on the height which cuts out 4 blank rows
            let y0 = ((py + 8) as f32 * 2.24 / WIDTH as f32) - 1.12;
            let mut x = 0.0;
            let mut y = 0.0;
            let mut iteration = 0;
            while (x * x + y * y) < 2.0 * 2.0 && iteration < max {
                let xtemp = x * x - y * y + x0;
                y = 2.0 * x * y + y0;
                x = xtemp;
                iteration = iteration + 1;
            }
            framebuffer[px + py * WIDTH] = iteration > 32;
        }
    }
}
