# zipwith

This is a simple zip-with implementation in Rust. Can be called as a function:
```
use zipwith::zip_with;
use std::cmp::max;

#[test]
fn zip_with_maxes_func() 
{
    let left = vec![0, 44, -12];
    let right = vec![4, 5, -8];
    let result: Vec<i8> = zip_with(left.into_iter(), right.into_iter(), max).collect();
    
    assert_eq!(result, vec![4, 44, -8]);
}
```
Or as a method:
```
use zipwith::IntoZipWith;
use std::cmp::max;

#[test]
fn zip_with_maxes_meth() 
{
    let left = vec![1, 2, 9];
    let right = vec![4, 5, 6];
    let result: Vec<u8> = left.into_iter().zip_with(right.into_iter(), max).collect();
    
    assert_eq!(result, vec![4, 5, 9]);
}
```