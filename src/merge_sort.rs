use std::cmp::Ordering;
use std::ops::Range;

fn merge_two_runs<T, C>(source: &[T], target: &mut [T], mid: usize, compare: &mut C)
    where T: Copy, C: FnMut(&T, &T) -> Ordering
{
    let mut i = 0;
    let mut j = mid;
    let mut k = 0;
    loop {
        if i == mid {
            target[k..].copy_from_slice(&source[j..]);
            break;
        }
        if j == source.len() {
            target[k..].copy_from_slice(&source[i..mid]);
            break;
        }
        match compare(&source[i], &source[j]) {
            Ordering::Less => {
                target[k] = source[i];
                i += 1;
            }
            _ => {
                target[k] = source[j];
                j += 1;
            }
        }
        k += 1;
    }
}

fn merge_sort_internal<T, C>(buffer0: &mut [T], buffer1: &mut [T],
                             compare: &mut C) -> usize
    where T: Copy, C: FnMut(&T, &T) -> Ordering
{
    let len = buffer0.len();
    debug_assert_eq!(buffer1.len(), len);
    if len == 1 {
        return 0;
    }

    let mid = len / 2;
    let x = merge_sort_internal(&mut buffer0[..mid], &mut buffer1[..mid], compare);
    let y = merge_sort_internal(&mut buffer0[mid..], &mut buffer1[mid..], compare);
    if x != y {
        let (source, target) = if x == 0 {
            (&buffer0[..mid], &mut buffer1[..mid])
        } else {
            (&buffer1[..mid], &mut buffer0[..mid])
        };
        target.copy_from_slice(source);
    }

    if y == 0 {
        merge_two_runs(buffer0, buffer1, mid, compare);
    } else {
        merge_two_runs(buffer1, buffer0, mid, compare);
    }
    1 - y
}

pub fn merge_sort<T, C>(input: &mut [T], compare: &mut C)
    where T: Copy, C: FnMut(&T, &T) -> Ordering
{
    if input.len() <= 1 {
        return;
    }

    let mut buffer = Vec::new();
    buffer.extend_from_slice(input);
    let x = merge_sort_internal(input, buffer.as_mut_slice(), compare);
    if x == 1 {
        input.copy_from_slice(buffer.as_slice());
    }
}

#[test]
fn test_merge_sort() {
    test_sort!(merge_sort);
}

fn scan_runs<T, C>(input: &[T], compare: &mut C) -> Vec<Range<usize>>
    where T: Copy, C: FnMut(&T, &T) -> Ordering
{
    let mut runs = Vec::new();
    let mut last_start = 0;
    for i in 1..input.len() {
        if compare(&input[i - 1], &input[i]) != Ordering::Greater {
            continue;
        }
        runs.push(last_start..i);
        last_start = i;
    }
    runs.push(last_start..input.len());
    runs
}

fn merge_runs<T, C, S>(buffer0: &mut [T], buffer1: &mut [T], runs: &[Range<usize>],
                       compare: &mut C, select_mid: &S) -> usize
    where T: Copy, C: FnMut(&T, &T) -> Ordering,
          S: Fn(&[Range<usize>]) -> usize
{
    if runs.len() <= 1 {
        return 0;
    }

    let last_run = runs.last().unwrap();
    let range = runs[0].start..last_run.end;
    if runs.len() == 2 {
        if runs[0].len() == 1 {
            // If the first run only contains one element, we already know
            // it is greater than the first element of the second run.
            let pivot = buffer0[runs[0].start];
            let mut i = runs[1].start + 1;
            while i < runs[1].end {
                if compare(&pivot, &buffer0[i]) != Ordering::Greater {
                    break;
                }
                i += 1;
            }
            buffer1[range.start..i - 1].copy_from_slice(&buffer0[runs[1].start..i]);
            buffer1[i - 1] = pivot;
            buffer1[i..range.end].copy_from_slice(&buffer0[i..runs[1].end]);
        } else {
            merge_two_runs(&buffer0[range.clone()], &mut buffer1[range],
                           runs[0].len(), compare);
        }
        return 1;
    }

    let mid = select_mid(runs);
    let x = merge_runs(buffer0, buffer1, &runs[..mid], compare, select_mid);
    let y = merge_runs(buffer0, buffer1, &runs[mid..], compare, select_mid);

    let x_range = runs[0].start..runs[mid].start;
    let y_range = runs[mid].start..last_run.end;
    let source_side;
    if x == y {
        source_side = x;
    } else {
        let copy_range = if x_range.len() < y_range.len() {
            source_side = y;
            x_range
        } else {
            source_side = x;
            y_range
        };
        let (source, target) = if source_side == 0 {
            (&buffer1[copy_range.clone()], &mut buffer0[copy_range])
        } else {
            (&buffer0[copy_range.clone()], &mut buffer1[copy_range])
        };
        target.copy_from_slice(source);
    }

    let (source, target) = if source_side == 0 {
        (buffer0, buffer1)
    } else {
        (buffer1, buffer0)
    };
    merge_two_runs(&source[range.clone()], &mut target[range],
                   runs[mid].start - runs[0].start, compare);

    1 - source_side
}

pub fn natural_merge_sort_internal<T, C, S>(input: &mut [T],
                                            compare: &mut C, select_mid: S)
    where T: Copy, C: FnMut(&T, &T) -> Ordering,
          S: Fn(&[Range<usize>]) -> usize
{
    let len = input.len();
    if len <= 1 {
        return;
    }

    let runs = scan_runs(input, compare);
    let mut buffer = Vec::new();
    buffer.extend_from_slice(input);
    let x = merge_runs(input, buffer.as_mut_slice(),
                       &runs, compare, &select_mid);
    if x == 1 {
        input.copy_from_slice(buffer.as_slice());
    }
}

pub fn natural_merge_sort<T, C>(input: &mut [T], compare: &mut C)
    where T: Copy, C: FnMut(&T, &T) -> Ordering
{
    natural_merge_sort_internal(input, compare, |runs| runs.len() / 2);
}

#[test]
fn test_natural_merge_sort() {
    test_sort!(natural_merge_sort);
}

pub fn natural_merge_sort2<T, C>(input: &mut [T], compare: &mut C)
    where T: Copy, C: FnMut(&T, &T) -> Ordering
{
    natural_merge_sort_internal(input, compare, |runs| {
        let last_run = runs.last().unwrap();
        let start = runs[0].start;
        let range = runs[0].start..last_run.end;
        let mid = range.len() / 2;
        runs.binary_search_by_key(&mid, |ref run| run.start - start)
            .unwrap_or_else(|pos| {
                if pos == runs.len() {
                    return pos - 1;
                }
                let before_mid = runs[pos - 1].start - start;
                let after_mid = runs[pos].start - start;
                if mid - before_mid < after_mid - mid {
                    pos - 1
                } else {
                    pos
                }
            })
    });
}

#[test]
fn test_natural_merge_sort2() {
    test_sort!(natural_merge_sort2);
}
