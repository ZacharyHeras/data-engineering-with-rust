fn main() {
    let mut arr1: [i32; 2] = [1, 2];
    println!("Original array, {:?}", arr1);
    println!("Original array reference {:p}", arr1.as_ptr());
    println!("Reference to first element in array {:p}", &arr1[0]);
    println!("Reference to second element in array {:p}", &arr1[1]);

    let idx1: usize = 0;
    let idx2: usize = 1;
    let arr2: &mut [i32] = number_swap(&mut arr1, idx1, idx2);
    println!(
        "Slice of array with the values swapped at idx {}, and idx {} {:?}",
        idx1, idx2, arr2
    );
    println!("Slice reference {:p} arr2", arr2.as_ptr());
    println!("Reference to first element in slice {:p}", &arr2[0]);
    println!("Reference to second element in slice {:p}", &arr2[1]);
}

fn number_swap(arr: &mut [i32], idx1: usize, idx2: usize) -> &mut [i32] {
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

    let new_arr: &mut [i32] = arr;
    new_arr[idx1] = val_idx2;
    new_arr[idx2] = val_idx1;

    return new_arr;
}

#[test]
fn test_number_swap1() {
    let mut arr1: [i32; 2] = [1, 2];
    assert!(number_swap(&mut arr1, 0, 1) == [2, 1])
}

#[test]
#[should_panic(expected = "idx1 out of range.")]
fn test_number_swap2() {
    let mut arr1: [i32; 2] = [1, 2];
    number_swap(&mut arr1, 4, 0);
}

#[test]
#[should_panic(expected = "idx2 out of range.")]
fn test_number_swap3() {
    let mut arr1: [i32; 2] = [1, 2];
    number_swap(&mut arr1, 0, 4);
}

#[test]
#[should_panic(expected = "Cannot swap element with itself.")]
fn test_number_swap4() {
    let mut arr1: [i32; 2] = [1, 2];
    number_swap(&mut arr1, 0, 0);
}
