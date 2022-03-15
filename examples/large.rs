fn main() {
    let framebuffer = vec![true; 128 * 64];
    let f = braillefb::Framebuffer::new(&framebuffer, 128, 64);
    for c in &f {
        print!("{}", c);
    }
}
