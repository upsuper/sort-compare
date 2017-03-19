extern crate rand;

use rand::Rng;

fn main() {
    let len = 256;
    let mut rng = rand::thread_rng();
    let mut input = Vec::with_capacity(len);
    for _ in 0..len {
        input.push(rng.gen::<i32>());
    }

    macro_rules! count_compare {
        ($sort_func:path) => {{
            let mut count = 0;
            let mut test_input = input.clone();
            $sort_func(&mut test_input, &mut |a, b| {
                count += 1;
                a.cmp(b)
            });
            println!(concat!(stringify!($sort_func), ": {}"), count);
        }}
    }

    println!("quick sort:");
    count_compare!(quick_sort::quick_sort);

    println!();
    println!("merge sort:");
    count_compare!(merge_sort::merge_sort);
    count_compare!(merge_sort::natural_merge_sort);
    count_compare!(merge_sort::natural_merge_sort2);

    println!();
    println!("heap sort:");
    count_compare!(heap_sort::heap_sort);
    count_compare!(heap_sort::heap_sort2);
}

#[cfg(test)]
macro_rules! test_sort {
    ($sort_func:path) => {
        use rand::{self, Rng};
        let mut rng = rand::thread_rng();
        for &len in [0, 1, 2, 3, 4, 12, 17, 27, 63, 64, 255, 256].iter() {
            // Test sorted vector.
            let mut input = Vec::with_capacity(len);
            for i in 0..len {
                input.push(i as i32);
            }
            let mut expected = input.clone();
            $sort_func(&mut input, &mut i32::cmp);
            assert_eq!(input, expected);

            // Test reversed sorted vector.
            for i in 0..len {
                input[i] = -(i as i32);
            }
            expected = input.clone();
            expected.reverse();
            $sort_func(&mut input, &mut i32::cmp);
            assert_eq!(input, expected);

            // Test random vector.
            for _ in 0..len {
                input.push(rng.gen::<i32>());
            }
            expected = input.clone();
            expected.sort();
            $sort_func(&mut input, &mut i32::cmp);
            assert_eq!(input, expected);
        }
    }
}

mod quick_sort;
mod merge_sort;
mod heap_sort;
