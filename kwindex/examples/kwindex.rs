fn main() {
        let index = kwindex::KWIndex::new()
            .extend_from_text("Hello world.");
        assert_eq!(2, index.len());
        assert_eq!(1, index.count_matches("world"));
        println!("{:?}",index)
}
