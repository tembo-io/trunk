pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const TRUNK_SAND_COLOR: Rgb = Rgb {
    r: 255,
    g: 247,
    b: 240,
};

pub fn indent(amount: u32) -> String {
    let mut new_amount = String::new();

    for _ in 0..amount {
        new_amount.push('\n');
    }
    new_amount
}
