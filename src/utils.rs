pub fn print_arr(data: &[usize], length: usize) {
    print!("Sorted List: [");
    for i in 0..length
    {
        if i != length-1 {
        print!("{}, ", data[i]);
        assert!(data[i] <= data[i+1]);
        } else {
        print!("{}", data[i]);
        }
    }
    print!("]");
}

pub fn quick_sort(array: &mut [usize], low: usize, high: usize) {
    if low < high {
        let pi: usize = partition(array, low, high); // find the pivot element such that: [Low .. < Pivot >  .. High]
        
        // Recurse left then right of pivot where low < high for each
        if pi >= 1 {
            quick_sort(array, low, pi - 1);  // from 0 to pivot
        } else {
            quick_sort(array, low, pi);
        };
        quick_sort(array, pi + 1, high);  // from pivot to end
    }
}

fn partition(array: &mut[usize], low: usize, high: usize) -> usize {
    let pivot = array[high];
    let mut i: isize = low as isize - 1;

    for j in low..high {
        if array[j] <= pivot {
        i += 1;
        swap(array, i as usize, j);
        }
    }

    let t: isize = i + 1;
    swap(array, t as usize, high);

    return t as usize;
}

fn swap(array: &mut [usize], a: usize, b: usize) {
    let t: usize = array[a];  // store value indexed by i
    array[a] = array[b];      // replace value with value pointed to by j
    array[b] = t;             // - then, swap value pointed to by j with i
}