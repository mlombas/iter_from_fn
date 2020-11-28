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

    #[test]
    fn test_inside_for() {
        let mut x = 1;
        let closure = || {
           let temp = x;

           //x = (x + 1)^2 each iteration
           x += 1;
           x *= x;

           temp
        };

        let mut v = Vec::new();
        for x in from(closure) {
           if x >= 10 { break }

           v.push(x);
        }

        assert_eq!(vec![1, 4], v);
    }
}
