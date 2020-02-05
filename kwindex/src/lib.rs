/// Each slice in this struct's list is a word in some
/// in-memory text document.
#[derive(Debug, Default, Clone)]
pub struct KWIndex<'a>(Vec<&'a str>);

impl<'a> KWIndex<'a> {
    /// Make a new empty target words list.
    pub fn new() -> Self {
        KWIndex(Vec::new())
    }

    /// Parse the `target` text and add the sequence of
    /// valid words contained in it to this `KWIndex`
    /// index.
    ///
    /// This is a "builder method": calls can be
    /// conveniently chained to build up an index.
    ///
    /// Words are separated by whitespace or punctuation,
    /// and consist of a span of one or more consecutive
    /// letters (any UTF-8 character in the "letter" class)
    /// with no internal punctuation.
    ///
    /// For example, the text
    ///
    /// ```text
    /// "It ain't over untïl it ain't, over."
    /// ```
    ///
    /// contains the sequence of words `"It"`, `"over"`,
    /// `"untïl"`, `"it"`, `"over"`.
    ///
    /// # Examples
    ///
    /// ```
    /// let index = kwindex::KWIndex::new()
    ///     .extend_from_text("Hello world.");
    /// assert_eq!(2, index.len());
    /// assert_eq!(1, index.count_matches("world"));
    /// ```
    pub fn extend_from_text(mut self, target: &'a str) -> Self {
        let alphabet_char = |x: char| x.is_alphabetic();
        let punctuations = |x| !alphabet_char(x);
        let trim_end_matches = |w: &'a str| w.trim_end_matches(punctuations);
        let words_collection = target
            .split_whitespace()
            .map(trim_end_matches)
            .filter(|w| w.chars().all(alphabet_char))
            .collect();
        self.0 = words_collection;
        self
    }

    /// Count the number of occurrences of the given `keyword`
    /// that are indexed by this `KWIndex`.
    pub fn count_matches(&self, keyword: &str) -> usize {
        let words_collection = &self.0;
        words_collection
            .iter()
            .filter(|word| **word == keyword)
            .count()
    }

    /// Count the number of words that are indexed by this
    /// `KWIndex`.
    pub fn len(&self) -> usize {
        let words_collection = &self.0;
        words_collection.len()
    }

    /// Is this index empty?
    pub fn is_empty(&self) -> bool {
        let words_collection = &self.0;
        words_collection.is_empty()
    }
}

#[test]
fn test_constructor() {
    let index = KWIndex::new();
    assert_eq!(0, index.len());
    assert_eq!(true, index.is_empty());
}

#[test]
fn test_word_seperator() {
    let index = KWIndex::new().extend_from_text("Hello world.");
    assert_eq!(2, index.len());
    assert_eq!(false, index.is_empty());
    assert_eq!(1, index.count_matches("world"));
}

#[test]
fn test_punctuations() {
    let index = KWIndex::new().extend_from_text("It ain't over untïl it ain't, over.");
    assert_eq!(5, index.len());
    assert_eq!(false, index.is_empty());
    assert_eq!(2, index.count_matches("over"));
}
