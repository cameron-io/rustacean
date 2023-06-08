use rand::Rng;

mod utils;

fn main() {
  println!("Running QuickSort test...");

  const N: usize = 20;
  let mut data: [usize; N] = [0; N];

  for i in 0..N {
      let r: usize = rand::thread_rng().gen_range(1..20);
      data[i] = r;
  }

  let length: usize = data.len();
  utils::quick_sort(&mut data, 0, length-1);

  utils::print_arr(&data, length)
}