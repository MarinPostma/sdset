use self::Minimums::*;

pub struct Union<'a, T: 'a> {
    slices: Vec<&'a [T]>,
}

impl<'a, T> Union<'a, T> {
    pub fn new(slices: Vec<&'a [T]>) -> Self {
        Union { slices }
    }
}

enum Minimums<T> {
    Nothing,
    One(T),
    Two(T, T),
}

/// Returns the index of the slices containing the minimum and the minimum values of two slices.
#[inline]
fn two_minimums<'a, T>(slices: &[&'a [T]]) -> Minimums<(usize, &'a T)>
where T: 'a + Ord
{
    let mut minimums: Minimums<(_, &T)> = Nothing;

    for (index, slice) in slices.iter().enumerate().filter(|(_, s)| !s.is_empty()) {
        let current = (index, &slice[0]);
        let (_, min) = current;

        minimums = match minimums {
            One(f) => if min < f.1 { Two(current, f) } else { Two(f, current) },
            Two(f, _) if min < f.1 => Two(current, f),
            Two(f, s) if min < s.1 => Two(f, current),
            Nothing => One(current),
            mins => mins,
        };
    }

    minimums
}

impl<'a, T: Ord + Clone> Union<'a, T> {
    pub fn extend_vec(mut self, output: &mut Vec<T>) {
        if let Some(slice) = self.slices.first() {
            output.reserve(slice.len());
        }

        loop {
            match two_minimums(&self.slices) {
                Two((i, _), (_, s)) => {
                    let len = output.len();
                    output.extend(self.slices[i].iter().take_while(|&e| e < s).cloned());
                    let add = output.len() - len;
                    self.slices[i] = &self.slices[i][add..];

                    output.push(s.clone());
                    for slice in self.slices.iter_mut().filter(|s| !s.is_empty()) {
                        if slice[0] == *s {
                            *slice = &slice[1..];
                        }
                    }
                },
                One((i, _)) => {
                    output.extend_from_slice(self.slices[i]);
                    break;
                },
                Nothing => break,
            }
        }
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
    fn no_slice() {
        let union_: Vec<i32> = Union::new(vec![]).into_vec();
        assert_eq!(&union_[..], &[]);
    }

    #[test]
    fn one_empty_slice() {
        let a: &[i32] = &[];

        let union_ = Union::new(vec![a]).into_vec();
        assert_eq!(&union_[..], &[]);
    }

    #[test]
    fn one_slice() {
        let a = &[1, 2, 3];

        let union_ = Union::new(vec![a]).into_vec();
        assert_eq!(&union_[..], &[1, 2, 3]);
    }

    #[test]
    fn two_slices_equal() {
        let a = &[1, 2, 3];
        let b = &[1, 2, 3];

        let union_ = Union::new(vec![a, b]).into_vec();
        assert_eq!(&union_[..], &[1, 2, 3]);
    }

    #[test]
    fn two_slices_little() {
        let a = &[1];
        let b = &[2];

        let union_ = Union::new(vec![a, b]).into_vec();
        assert_eq!(&union_[..], &[1, 2]);
    }

    #[test]
    fn two_slices() {
        let a = &[1, 2, 3];
        let b = &[2, 3, 4];

        let union_ = Union::new(vec![a, b]).into_vec();
        assert_eq!(&union_[..], &[1, 2, 3, 4]);
    }

    #[test]
    fn three_slices() {
        let a = &[1, 2, 3];
        let b = &[2, 3, 4];
        let c = &[3, 4, 5];

        let union_ = Union::new(vec![a, b, c]).into_vec();
        assert_eq!(&union_[..], &[1, 2, 3, 4, 5]);
    }

    #[bench]
    fn bench_two_slices_big(bench: &mut Bencher) {
        let a: Vec<_> = (0..100).collect();
        let b: Vec<_> = (1..101).collect();

        bench.iter(|| {
            let union_ = Union::new(vec![&a, &b]).into_vec();
            test::black_box(|| union_);
        });
    }

    #[bench]
    fn bench_two_slices_big2(bench: &mut Bencher) {
        let a: Vec<_> = (0..100).collect();
        let b: Vec<_> = (51..151).collect();

        bench.iter(|| {
            let union_ = Union::new(vec![&a, &b]).into_vec();
            test::black_box(|| union_);
        });
    }

    #[bench]
    fn bench_two_slices_big3(bench: &mut Bencher) {
        let a: Vec<_> = (0..100).collect();
        let b: Vec<_> = (100..200).collect();

        bench.iter(|| {
            let union_ = Union::new(vec![&a, &b]).into_vec();
            test::black_box(|| union_);
        });
    }

    fn sort_dedup<T: Ord>(x: &mut Vec<T>) {
        x.sort_unstable();
        x.dedup();
    }

    quickcheck! {
        fn qc_union(xss: Vec<Vec<i32>>) -> bool {
            use std::collections::BTreeSet;
            use std::iter::FromIterator;

            // FIXME temporary hack (can have mutable parameters!)
            let mut xss = xss;

            for xs in &mut xss {
                sort_dedup(xs);
            }

            let x = {
                let xss = xss.iter().map(|xs| xs.as_slice()).collect();
                Union::new(xss).into_vec()
            };

            let mut y = BTreeSet::new();
            for v in xss {
                let x = BTreeSet::from_iter(v.iter().cloned());
                y = y.union(&x).cloned().collect();
            }
            let y: Vec<_> = y.into_iter().collect();

            x.as_slice() == y.as_slice()
        }
    }
}