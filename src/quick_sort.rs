pub fn run<T: PartialOrd>(vec: &mut [T]) {
    match vec.len() {
        0 | 1 => return,
        2 => {
            if &vec[0] > &vec[1] {
                vec.swap(0, 1);
            }
        }
        _ => quick_sort_in_place(vec),
    }
}

fn quick_sort_in_place<T: PartialOrd>(vec: &mut [T]) {
    let pivot_index = vec.len() - 1;
    let mut right = pivot_index - 1;
    let mut left = 0;
    loop {
        while vec[left] < vec[pivot_index] && left < pivot_index {
            left += 1;
        }
        while vec[right] > vec[pivot_index] && right > left && right > 0 {
            right -= 1;
        }
        if left >= right {
            if left != pivot_index {
                vec.swap(left, pivot_index)
            }
            run(&mut vec[..left]);
            run(&mut vec[(left + 1)..]);
            break;
        }
        vec.swap(left, right);
    }
}

#[cfg(test)]
mod test {
    use crate::quick_sort;

    #[test]
    fn test1() {
        let mut vec = vec![7, 2, 4, 3, 1];
        quick_sort::run(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 7]);
    }

    #[test]
    fn test2() {
        let mut vec = vec![3, 7, 8, 5, 2, 1, 9, 5, 4];
        quick_sort::run(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 5, 7, 8, 9]);
    }

    #[test]
    fn test3() {
        let mut vec = vec![3, 2, 1, 4, 7, 8, 5];
        quick_sort::run(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 7, 8]);
    }

    #[test]
    fn test4() {
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        quick_sort::run(&mut vec);
        assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
