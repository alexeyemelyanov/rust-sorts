pub fn quick_sort<T: PartialOrd>(vec: &mut [T]) {
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

pub fn quick_sort_in_place<T: PartialOrd>(vec: &mut [T]) {
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
            quick_sort(&mut vec[..left]);
            quick_sort(&mut vec[(left + 1)..]);
            break;
        }
        vec.swap(left, right);
    }
}
