mod merge_sort;
mod quick_sort;

fn main() {
    let mut vec = vec![7, 2, 4, 3, 1];
    quick_sort::quick_sort(&mut vec);
    println!("{:?}", vec);

    vec = vec![3, 7, 8, 5, 2, 1, 9, 5, 4];
    quick_sort::quick_sort(&mut vec);
    println!("{:?}", vec);

    vec = vec![3, 2, 1, 4, 7, 8, 5];
    quick_sort::quick_sort(&mut vec);
    println!("{:?}", vec);

    vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    quick_sort::quick_sort(&mut vec);
    println!("{:?}", vec);

    let mut vec = vec![7, 2, 4, 3, 1];
    merge_sort::merge_sort(&mut vec);
    println!("{:?}", vec);

    vec = vec![3, 7, 8, 5, 2, 1, 9, 5, 4];
    merge_sort::merge_sort(&mut vec);
    println!("{:?}", vec);

    vec = vec![3, 2, 1, 4, 7, 8, 5];
    merge_sort::merge_sort(&mut vec);
    println!("{:?}", vec);

    vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    merge_sort::merge_sort(&mut vec);
    println!("{:?}", vec);
}
