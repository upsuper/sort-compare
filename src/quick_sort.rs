use std::cmp::Ordering;

pub fn quick_sort<T, C>(input: &mut [T], compare: &mut C)
    where T: Copy, C: FnMut(&T, &T) -> Ordering
{
    if input.len() <= 1 {
        return;
    }

    let len = input.len();
    let mut i = 0;
    let mut j = len - 1;
    let mid = input[len / 2];
    loop {
        while i < len && compare(&input[i], &mid) == Ordering::Less {
            i += 1;
        }
        while j > 0 && compare(&input[j], &mid) == Ordering::Greater {
            j -= 1;
        }
        if i < j {
            input.swap(i, j);
            i += 1;
            j -= 1;
        } else {
            break;
        }
    }
    quick_sort(&mut input[..i], compare);
    quick_sort(&mut input[i..], compare);
}

#[test]
fn test_quick_sort() {
    test_sort!(quick_sort);
}
