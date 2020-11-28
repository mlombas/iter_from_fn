pub struct InfiniteFromFn<F: FnMut() -> T, T> {
    f: F,
}

pub fn from<F, T>(f: F) -> InfiniteFromFn<F, T> 
    where F: FnMut() -> T {
    InfiniteFromFn { f }
}

impl<F: FnMut() -> T, T> Iterator for InfiniteFromFn<F, T> {
    type Item = T;
    
    fn next(&mut self) -> Option<T> {
        Some((self.f)())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ones() {
        let f = || 1;
        let it = from(f);
        assert_eq!(vec![1; 10], it.take(10).collect::<Vec<_>>());
    }

    #[test]
    fn test_mutable() {
        let mut x = 0;
        let f = || { x += 1; x };
        let counter = from(f);
        assert_eq!(vec![1,2,3,4,5], counter.take(5).collect::<Vec<_>>());
    }
}
