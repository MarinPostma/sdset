mod union;
mod intersection;
mod difference;

pub use self::union::Union;
pub use self::intersection::Intersection;
pub use self::difference::Difference;

pub struct OpBuilder<'a, T: 'a> {
    slices: Vec<&'a [T]>,
}

impl<'a, T> OpBuilder<'a, T> {
    pub fn new() -> Self {
        Self { slices: Vec::new() }
    }

    pub fn from_vec(slices: Vec<&'a [T]>) -> Self {
        Self { slices }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { slices: Vec::with_capacity(capacity) }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.slices.reserve(additional);
    }

    pub fn add(mut self, slice: &'a [T]) -> Self {
        self.push(slice);
        self
    }

    pub fn push(&mut self, slice: &'a [T]) {
        self.slices.push(slice);
    }

    pub fn union(self) -> Union<'a, T> {
        Union::new(self.slices)
    }

    pub fn intersection(self) -> Intersection<'a, T> {
        Intersection::new(self.slices)
    }

    pub fn difference(self) -> Difference<'a, T> {
        Difference::new(self.slices)
    }
}