use std::cmp::Ordering;

fn sink<T, C>(heap: &mut [T], pos: usize, compare: &mut C)
    where T: Copy, C: FnMut(&T, &T) -> Ordering
{
    let left = pos * 2 + 1;
    let right = left + 1;
    if left >= heap.len() {
        return;
    }
    let max_child = if right >= heap.len() {
        left
    } else {
        match compare(&heap[left], &heap[right]) {
            Ordering::Less => right,
            _ => left,
        }
    };
    if compare(&heap[pos], &heap[max_child]) == Ordering::Less {
        heap.swap(pos, max_child);
        sink(heap, max_child, compare);
    }
}

fn build_heap<T, C>(input: &mut [T], compare: &mut C)
    where T: Copy, C: FnMut(&T, &T) -> Ordering
{
    let len = input.len();
    for i in (0..len / 2).rev() {
        sink(input, i, compare);
    }
}

pub fn heap_sort<T, C>(input: &mut [T], compare: &mut C)
    where T: Copy, C: FnMut(&T, &T) -> Ordering
{
    build_heap(input, compare);
    for i in (1..input.len()).rev() {
        input.swap(0, i);
        sink(&mut input[..i], 0, compare);
    }
}

#[test]
fn test_heap_sort() {
    test_sort!(heap_sort);
}

fn fill_vacancy<T, C>(heap: &mut [T], pos: usize, compare: &mut C) -> usize
    where T: Copy, C: FnMut(&T, &T) -> Ordering
{
    let left = pos * 2 + 1;
    let right = left + 1;
    if left >= heap.len() {
        return pos;
    }
    if right >= heap.len() {
        heap[pos] = heap[left];
        return left;
    }
    let max_child = match compare(&heap[left], &heap[right]) {
        Ordering::Less => right,
        _ => left,
    };
    heap[pos] = heap[max_child];
    return fill_vacancy(heap, max_child, compare);
}

fn float<T, C>(heap: &mut [T], pos: usize, compare: &mut C)
    where T: Copy, C: FnMut(&T, &T) -> Ordering
{
    if pos == 0 {
        return;
    }
    let parent = (pos - 1) / 2;
    if compare(&heap[pos], &heap[parent]) == Ordering::Greater {
        heap.swap(pos, parent);
        float(heap, parent, compare);
    }
}

pub fn heap_sort2<T, C>(input: &mut [T], compare: &mut C)
    where T: Copy, C: FnMut(&T, &T) -> Ordering
{
    build_heap(input, compare);
    for i in (1..input.len()).rev() {
        let next_sorted = input[0];
        let vacancy = fill_vacancy(&mut input[..i], 0, compare);
        if vacancy != i {
            input[vacancy] = input[i];
            float(&mut input[..i], vacancy, compare);
        }
        input[i] = next_sorted;
    }
}

#[test]
fn test_heap_sort2() {
    test_sort!(heap_sort2);
}
