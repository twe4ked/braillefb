macro_rules! framebuffer {
    (#) => {true};
    (.) => {false};
    ($($c:tt)+) => {vec![
        $(framebuffer!($c)),+
    ]}
}

fn main() {
    let framebuffer = framebuffer![
        # . # #
        # . . #
        # . # #
        # # . .
        # # . #
        # # . #
        . . # #
        # . # .
    ];
    let f = braillefb::Framebuffer::new(&framebuffer, 4, 8);
    print!("{}", f.into_iter().collect::<String>());
}
