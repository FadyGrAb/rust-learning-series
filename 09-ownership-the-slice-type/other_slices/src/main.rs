fn main() {
    let col = [10, 20, 30, 40, 50];
    let slice = &col[2..4];
    assert_eq!(slice, &[30, 40]);
}
