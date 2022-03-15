mod mandelbrot {
    pub struct MandelbrotSet {
        max_iterations: u32,
        range: (f64, f64),
    }

    impl Default for MandelbrotSet {
        fn default() -> Self {
            Self {
                max_iterations: 50,
                range: (-2.0, 2.0),
            }
        }
    }

    impl MandelbrotSet {
        pub fn generate<F>(&self, width: usize, height: usize, mut callback: F)
        where
            F: FnMut(usize, usize, u32),
        {
            let min_im = -1.2;
            let max_im = min_im + (self.range.1 - self.range.0) * height as f64 / width as f64;
            let re_factor = (self.range.1 - self.range.0) / (width - 1) as f64;
            let im_factor = (max_im - min_im) / (height - 1) as f64;

            for y in 0..height {
                let c_im = max_im - y as f64 * im_factor;

                for x in 0..width {
                    let c_re = self.range.0 + x as f64 * re_factor;
                    let mut z_re = c_re;
                    let mut z_im = c_im;

                    let mut color = 0;
                    for iterations in 0..self.max_iterations {
                        let z_re2 = z_re * z_re;
                        let z_im2 = z_im * z_im;

                        if iterations != self.max_iterations {
                            color = 50 + iterations * 10 % 255;
                        } else {
                            color = 0;
                        }

                        if z_re2 + z_im2 > 4. {
                            break;
                        }

                        z_im = 2. * z_re * z_im + c_im;
                        z_re = z_re2 - z_im2 + c_re;
                    }

                    callback(x, y, color)
                }
            }
        }
    }
}

fn main() {
    let mandelbrot_set = mandelbrot::MandelbrotSet::default();

    let width = 128;
    let height = 96;
    let mut framebuffer = vec![false; width * height];
    mandelbrot_set.generate(width, height, |x, y, color| {
        if color > 256 {
            framebuffer[x + y * width] = true;
        }
    });

    let f = braillefb::Framebuffer::new(&framebuffer, width, height);
    print!("{}", f.into_iter().collect::<String>());
}
