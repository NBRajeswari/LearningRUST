/// Trait for applying unique() to an iterator
pub trait Unique<T> {
    fn unique<P>(&mut self, pred: P) -> Option<T>
    where
        P: FnMut(&T) -> bool;
}

impl<T, I> Unique<T> for I
where
    I: Iterator<Item = T>,
{
    fn unique<P>(&mut self, pred: P) -> Option<T>
    where
        P: FnMut(&T) -> bool,
    {
        // collect the items that satisfyes the predicate
        let mut filtered_items = self.filter(pred).collect::<Vec<_>>();

        // if exactly one item matches the pred
        // return Some(elem) else return None
        if filtered_items.len() == 1 {
            return filtered_items.pop();
        }

        None
    }
}

#[test]
fn test_unique() {
    let mut nums = vec![];
    let even: fn(&&usize) -> bool = |&&n| n % 2 == 0;
    assert_eq!(None, nums.iter().unique(even));
    nums.push(1);
    assert_eq!(None, nums.iter().unique(even));
    nums.push(0);
    assert_eq!(Some(&0), nums.iter().unique(even));
    nums.push(3);
    nums.push(2);
    nums.push(5);
    assert_eq!(None, nums.iter().unique(even));
}
