# Iter from Function
As simple as it sounds. You have a function, you want to create an infinite iterable
that repeatedly calls said function over and over? This package is for you

# Usage
If you have a function, just pass it to the `from` function of the package. 
You can use take to get a specified number of elements.

```Rust
extern crate iter_from_fn;

fn generate_one() -> u64 {
   1
}

assert_eq!(
   vec![1; 10],
   iter_from_fn::rom(generate_one).take(10).collect::<Vec<_>>()
);

let mut x = 0;
let closure = || { x += 1; x }
assert_eq!(
   vec![1,2,3,4,5],
   iter_from_fn::from(closure).take(5).collect::<Vec<_>>()
);
```

You can also use it in a for loop and break

```Rust
extern crate iter_from_fn;

let mut x = 1;
let closure = || {
  let temp = x;

  //x = (x + 1)^2 each iteration
  x += 1;
  x *= x;

  temp
};

let mut v = Vec::new();
for x in iter_from_fn::from(closure) {
  if x >= 10 { break }

  v.push(x);
}

assert_eq!(vec![1, 4], v);
```

Use a closure if you need to bind values to arguments.

```Rust
   extern crate iter_from_fn;

   let vec_100_capacity = || Vec::with_capacity(100);
   let mut it = iter_from_fn::from(vec_100_capacity);

   assert_eq!(100, it.next().capacity());
```
