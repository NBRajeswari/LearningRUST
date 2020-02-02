fn main() {
    let index = kwindex::KWIndex::new().extend_from_text("Hello world.");
    assert_eq!(2, index.len());
    assert_eq!(1, index.count_matches("world"));
    println!("{:?}", index);
    let index2 = kwindex::KWIndex::new().extend_from_text("It ain't over untïl it ain't, over.");
    assert_eq!(5, index2.len());
    assert_eq!(2, index2.count_matches("over"));
    println!("{:?}", index2)
}
