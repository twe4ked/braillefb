# braillefb

A framebuffer that takes a `&[bool]` slice and returns 2x4 "dot" (pixel) braille `char`s.

## Example

```rust
// ⣇⠽
let framebuffer = vec![
    true, false, true, true,
    true, false, false, true,
    true, false, true, true,
    true, true, false, false,
];

let f = Framebuffer::new(&framebuffer, 4, 4);

// Get individual braille `char`s
assert_eq!(Some('⣇'), f.get(0));

// As an iterator
let output: String = f.into_iter().collect();
assert_eq!("⣇⠽\n", &output);
```

License: MIT OR Apache-2.0
