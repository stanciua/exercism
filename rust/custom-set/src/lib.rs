
use std::cmp::PartialEq;

#[derive(Debug, Clone)]
pub struct CustomSet<T> {
    elems: Vec<T>,
}

impl<T> PartialEq for CustomSet<T>
    where T: PartialEq + Clone + Copy
{
    fn eq(&self, other: &CustomSet<T>) -> bool {
        self.elems.len() == other.elems.len() &&
        self.elems.iter().map(|e| other.contains(e)).all(|x| x)
    }
}
impl<T> CustomSet<T>
    where T: PartialEq + Clone + Copy
{
    pub fn new(vec: Vec<T>) -> Self {
        CustomSet {
            elems: vec.into_iter().fold(Vec::new(), |mut acc, val| {
                if !acc.contains(&val) {
                    acc.push(val);
                }
                acc
            }),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.elems.len() == 0
    }

    pub fn contains(&self, elem: &T) -> bool {
        self.elems.contains(elem)
    }

    pub fn is_subset(&self, set: &CustomSet<T>) -> bool {
        if self.elems.is_empty() {
            true
        } else {
            self.elems.iter().map(|e| set.contains(e)).all(|x| x)
        }

    }

    pub fn is_disjoint(&self, set: &CustomSet<T>) -> bool {
        if self.elems.is_empty() {
            true
        } else {
            self.elems.iter().map(|e| !set.contains(e)).all(|x| x)
        }

    }

    pub fn add(&mut self, elem: T) {
        if !self.elems.contains(&elem) {
            self.elems.push(elem);
        }

    }

    pub fn intersection(&self, other: &CustomSet<T>) -> Self {
        CustomSet {
            elems: self.elems
                .iter()
                .cloned()
                .map(|e| (e, other.contains(&e)))
                .filter(|&(_, shared)| shared)
                .map(|(e, _)| e)
                .collect(),
        }
    }

    pub fn difference(&self, other: &CustomSet<T>) -> Self {
        CustomSet {
            elems: self.elems
                .iter()
                .cloned()
                .map(|e| (e, !other.contains(&e)))
                .filter(|&(_, noshrd)| noshrd)
                .map(|(e, _)| e)
                .collect(),
        }
    }

    pub fn union(&self, other: &CustomSet<T>) -> Self {
        self.elems.iter().chain(other.elems.iter()).fold(CustomSet::new(vec![]), |mut acc, val| {
            acc.add(*val);
            acc
        })

    }
}
