use rand::Rng;

fn swap(array: &mut [usize], a: usize, b: usize)
{
  let t: usize = array[a];  // store value indexed by i
  array[a] = array[b];      // replace value with value pointed to by j
  array[b] = t;             // - then, swap value pointed to by j with i
}

fn partition(array: &mut[usize], low: usize, high: usize) -> usize
{
  let pivot = array[high];
  let mut i: isize = low as isize - 1;

  for j in low..high
  {
    if array[j] <= pivot
    {
      i += 1;
      swap(array, i as usize, j);
    }
  }

  let t: isize = i + 1;
  swap(array, t as usize, high);

  return t as usize;
}

fn quick_sort(array: &mut [usize], low: usize, high: usize)
{
  if low < high
  {
    let pi: usize = partition(array, low, high); // find the pivot element such that: [Low .. < Pivot >  .. High]
    
    // Recurse left then right of pivot where low < high for each
    quick_sort(array, low, pi - 1);  // from 0 to pivot
    quick_sort(array, pi + 1, high);  // from pivot to end
  }
}

fn main()
{
  println!("Running QuickSort test...");

  const N: usize = 20;
  let mut data: [usize; N] = [0; N];

  for i in 0..N
  {
      let r: usize = rand::thread_rng().gen_range(1..20);
      data[i] = r;
  }

  let length: usize = data.len();
  quick_sort(&mut data, 0, length-1);

  print!("Sorted List: [");
  for i in 0..N
  {
    if i != N-1 {
      print!("{}, ", data[i]);
      assert!(data[i] <= data[i+1]);
    } else {
      print!("{}", data[i]);
    }
  }
  print!("]");
}