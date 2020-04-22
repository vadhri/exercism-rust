
use core::ops::Rem;

pub struct Matcher<T> {
    matcher: Box<dyn Fn(T) -> bool>,
    subs: String
}
impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: ToString
    {
        Self {
            matcher: Box::new(matcher),
            subs: subs.to_string(),
        }
    }
}

pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>
}

impl<T: Copy + Clone + ToString> Fizzy<T> where T: ToString {
    pub fn new() -> Self {
        Fizzy {
            matchers: Vec::new()
        }
    }

    pub fn add_matcher(mut self, _matcher: Matcher<T>) -> Self {
        self.matchers.push(_matcher);
        self
    }

    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |item| self.apply_each_item(item))
    }

    fn apply_each_item(&self, item: T) -> String {
        let s = self
            .matchers
            .iter()
            .filter(|matcher| (matcher.matcher)(item))
            .map(|matcher| matcher.subs.clone())
            .collect::<String>();

        if s.is_empty() {
            item.to_string()
        } else {
            s
        }
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T> where
    T: Copy + Default + From<u8> + PartialEq + Rem<Output = T> + 'static, T: std::fmt::Display
{
        Fizzy::new()
            .add_matcher(Matcher::new(|n: T| n % 3.into() == T::default(), "fizz"))
            .add_matcher(Matcher::new(|n: T| n % 5.into() == T::default(), "buzz"))
}
