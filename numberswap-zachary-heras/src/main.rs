const CAPACITY: usize = 2;

fn main() {
    let arr1: [i32; CAPACITY] = [1, 2];
    let arr2: [i32; CAPACITY] = number_swap(arr1, 0, 1);
    println!("{:?}, {:?}", arr1, arr2);
}

fn number_swap(arr: [i32; CAPACITY], idx1: usize, idx2: usize) -> [i32; CAPACITY] {
    if idx1 > arr.len() - 1 {
        panic!("idx1 out of range.")
    }

    if idx2 > arr.len() - 1 {
        panic!("idx2 out of range.")
    }

    if idx1 == idx2 {
        panic!("Cannot swap element with itself.")
    }

    let val_idx1: i32 = arr[idx1];
    let val_idx2: i32 = arr[idx2];

    let mut new_arr: [i32; CAPACITY] = arr;
    new_arr[idx1] = val_idx2;
    new_arr[idx2] = val_idx1;

    return new_arr;
}

#[test]
fn test_number_swap1() {
    let arr1: [i32; CAPACITY] = [1, 2];
    assert!(number_swap(arr1, 0, 1) == [2, 1])
}

#[test]
#[should_panic(expected = "idx1 out of range.")]
fn test_number_swap2() {
    let arr1: [i32; CAPACITY] = [1, 2];
    number_swap(arr1, 4, 0);
}

#[test]
#[should_panic(expected = "idx2 out of range.")]
fn test_number_swap3() {
    let arr1: [i32; CAPACITY] = [1, 2];
    number_swap(arr1, 0, 4);
}

#[test]
#[should_panic(expected = "Cannot swap element with itself.")]
fn test_number_swap4() {
    let arr1: [i32; CAPACITY] = [1, 2];
    number_swap(arr1, 0, 0);
}
