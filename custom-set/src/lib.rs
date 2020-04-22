use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    set: Vec<T>
}

impl<T: std::fmt::Debug +  std::clone::Clone +  std::cmp::Ord> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut input = _input.clone().to_vec();
        input.sort();
        input.dedup();

        CustomSet {
            set: input
        }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.set.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        self.set.push(_element);
        self.set.sort();
        self.set.dedup();
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        if self.set.len() == 0 {
            true
        } else if _other.set.len() == 0 {
            false
        }
        else {
            match _other.set.windows(self.set.len()).position(|window| window.to_vec() == self.set) {
                Some(_x) => true,
                None => false
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        println!("{:?} - {:?}", self.set, _other.set);

        if self.set.len() == 0 && _other.set.len() == 0 {
            true
        } else {
            self.intersection(_other).set.len() == 0
        }
    }

    pub fn intersection(&self, _other: &Self) -> Self {
        let mut intersection_vec = Vec::new();

        for _element in self.set.clone() {
            match _other.set.contains(&_element) {
                true => {
                    intersection_vec.push(_element);
                }
                false => {

                }
            }
        }

        CustomSet::new(intersection_vec.as_slice())
    }

    pub fn difference(&self, _other: &Self) -> Self {
        let mut difference_vec = Vec::new();

        for _element in self.set.clone() {
            match _other.set.contains(&_element) {
                true => {

                }
                false => {
                    difference_vec.push(_element);
                }
            }
        }

        CustomSet::new(difference_vec.as_slice())
    }

    pub fn union(&self, _other: &Self) -> Self {
        let mut out = self.set.clone();
        let mut _other_set = _other.set.clone();
        out.append(&mut _other_set);
        CustomSet::new(out.as_slice())
    }
}
