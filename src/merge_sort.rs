pub fn run<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() > 1 {
        merge_sort_in_place(arr, 0, arr.len() - 1)
    }
}

fn merge_sort_in_place<T: PartialOrd>(arr: &mut [T], start: usize, end: usize) {
    match end - start {
        0 => {}
        1 => {
            if arr[end] < arr[start] {
                arr.swap(start, end);
            }
        }
        diff => {
            let mut border = start + diff / 2;
            merge_sort_in_place(arr, start, border);
            border += 1;
            if border < end {
                merge_sort_in_place(arr, border, end);
            }
            merge_in_place(arr, start, end, border);
        }
    }
}

fn merge_in_place<T: PartialOrd>(arr: &mut [T], start: usize, end: usize, border: usize) {
    let bp1 = border + 1;
    for ptr in start..border {
        if arr[ptr] > arr[border] {
            arr.swap(ptr, border);
            if border < end && arr[border] > arr[bp1] {
                merge_sort_in_place(arr, border, end);
            }
        }
    }
}
