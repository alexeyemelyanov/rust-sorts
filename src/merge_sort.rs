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

#[cfg(test)]
mod test {
    use crate::merge_sort;

    #[test]
    fn test1() {
        let mut vec = vec![7, 2, 4, 3, 1];
        merge_sort::run(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 7]);
    }

    #[test]
    fn test2() {
        let mut vec = vec![3, 7, 8, 5, 2, 1, 9, 5, 4];
        merge_sort::run(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 5, 7, 8, 9]);
    }

    #[test]
    fn test3() {
        let mut vec = vec![3, 2, 1, 4, 7, 8, 5];
        merge_sort::run(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 7, 8]);
    }

    #[test]
    fn test4() {
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        merge_sort::run(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
