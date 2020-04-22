/// What should the type of _function be?
pub fn map<T, U, F>(input: Vec<T>, mut f: F) -> Vec<U> where F: FnMut(T) -> U  {
    let mut output: Vec<U> = Vec::new();

    for item in input {
        output.push(f(item));
    }

    output
}
