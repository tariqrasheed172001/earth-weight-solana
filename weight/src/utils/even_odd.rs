pub struct EvenInput {
    pub a: i32,
    pub b: i32
}

pub fn even_odd(i: EvenInput) -> i32 {
    i.a % 2
}