use std::cmp::{self, Ordering};

pub struct Union<'a, T: 'a> {
    a: &'a [T],
    b: &'a [T],
}

impl<'a, T: 'a> Union<'a, T> {
    pub fn new(a: &'a [T], b: &'a [T]) -> Self {
        Union { a, b }
    }
}

impl<'a, T: 'a + Ord + Clone> Union<'a, T> {
    pub fn extend_vec(mut self, output: &mut Vec<T>) {
        let min_len = cmp::max(self.a.len(), self.b.len());
        output.reserve(min_len);

        while !self.a.is_empty() && !self.b.is_empty() {
            match self.a[0].cmp(&self.b[0]) {
                Ordering::Less => {
                    output.push(self.a[0].clone());
                    self.a = &self.a[1..];
                },
                Ordering::Equal => {
                    output.push(self.a[0].clone());
                    self.a = &self.a[1..];
                    self.b = &self.b[1..];
                },
                Ordering::Greater => {
                    output.push(self.b[0].clone());
                    self.b = &self.b[1..];
                },
            }
        }

        output.extend(self.a.iter().cloned());
        output.extend(self.b.iter().cloned());
    }

    pub fn into_vec(self) -> Vec<T> {
        let mut vec = Vec::new();
        self.extend_vec(&mut vec);
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{self, Bencher};

    #[test]
    fn union_two_slices_easy() {
        let a = &[1, 2, 3];
        let b = &[2, 3, 4];

        let union: Vec<_> = Union::new(a, b).into_vec();

        assert_eq!(&union, &[1, 2, 3, 4]);
    }

    #[test]
    fn union_two_slices_second_empty() {
        let a = &[1, 2, 3];
        let b = &[];

        let union: Vec<_> = Union::new(a, b).into_vec();

        assert_eq!(&union, &[1, 2, 3]);
    }

    #[test]
    fn union_two_slices_first_empty() {
        let a = &[];
        let b = &[2, 3, 4];

        let union: Vec<_> = Union::new(a, b).into_vec();

        assert_eq!(&union, &[2, 3, 4]);
    }

    #[test]
    fn union_two_slices_same_elem() {
        let a = &[1];
        let b = &[1];

        let union: Vec<_> = Union::new(a, b).into_vec();

        assert_eq!(&union, &[1]);
    }

    #[bench]
    fn union_bench_two_slices_big(bench: &mut Bencher) {
        let a: Vec<_> = (0..100).collect();
        let b: Vec<_> = (1..101).collect();

        bench.iter(|| {
            let union: Vec<_> = Union::new(&a, &b).into_vec();
            test::black_box(|| union);
        });
    }

    #[bench]
    fn union_bench_two_slices_big2(bench: &mut Bencher) {
        let a: Vec<_> = (0..100).collect();
        let b: Vec<_> = (51..151).collect();

        bench.iter(|| {
            let union_ = Union::new(&a, &b).into_vec();
            test::black_box(|| union_);
        });
    }

    #[bench]
    fn union_bench_two_slices_big3(bench: &mut Bencher) {
        let a: Vec<_> = (0..100).collect();
        let b: Vec<_> = (100..200).collect();

        bench.iter(|| {
            let union_ = Union::new(&a, &b).into_vec();
            test::black_box(|| union_);
        });
    }

    fn sort_dedup<T: Ord>(x: &mut Vec<T>) {
        x.sort_unstable();
        x.dedup();
    }

    quickcheck! {
        fn qc_union(a: Vec<i32>, b: Vec<i32>) -> bool {
            use std::collections::BTreeSet;
            use std::iter::FromIterator;

            let mut a = a;
            let mut b = b;

            sort_dedup(&mut a);
            sort_dedup(&mut b);

            let x = Union::new(&a, &b).into_vec();

            let a = BTreeSet::from_iter(a);
            let b = BTreeSet::from_iter(b);
            let y = a.union(&b);
            let y: Vec<_> = y.cloned().collect();

            x.as_slice() == y.as_slice()
        }
    }
}