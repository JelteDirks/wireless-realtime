pub fn delay() {
    for _ in 1..100000 {
        core::hint::black_box(5);
    }
}
