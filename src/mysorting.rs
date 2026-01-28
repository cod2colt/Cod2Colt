use crate::myrust::MyPrint;

// bubble sort
pub fn sort_bubble(print_out: &mut MyPrint) {
    let mut arr = vec![5, 1, 4, 2, 8];
    let mut st = format!("0: {:?}", arr);
    print_out.print_line(st);

    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
        st = format!("{}: {:?}", i + 1, arr);
        print_out.print_line(st);
    }
}

// insert sort
pub fn sort_insert(print_out: &mut MyPrint) {
    let mut arr = vec![5, 1, 4, 2, 8];
    let mut st = format!("0: {:?}", arr);
    print_out.print_line(st);

    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;

        st = format!("{}: {:?}", i, arr);
        print_out.print_line(st);
    }
}
