pub trait Mergeable {
    fn merge(&mut self, other: Self);
}

impl<T> Mergeable for Vec<T> {
    fn merge(&mut self, other: Self) {
        self.extend(other);
    }
}
