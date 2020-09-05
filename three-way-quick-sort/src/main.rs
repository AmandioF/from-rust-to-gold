use std::cmp::Ordering;

fn quicksort(arr: &mut [i32], left: usize, right: usize) {
    if right <= left {
        return;
    }
    let mut lt = left;
    let mut gt = right;
    
    let pivot = arr[left];
    
    let mut i = left + 1;
    while i <= gt {
        match arr[i].cmp(&pivot) {
            Ordering::Less => {
                arr.swap(lt, i);
                lt = lt + 1;
                i = i + 1;
            },
            Ordering::Greater => {
                arr.swap(i, gt);
                gt = gt - 1;
            },
            Ordering::Equal => {
                i = i + 1;
            }
        }
    }

    if lt != 0 { quicksort(arr, left, lt-1); }
    quicksort(arr, gt+1, right);
}
fn main() {
    let mut arr: Vec<i32> = {
                            let mut buf = String::new();
                            std::io::stdin().read_line(&mut buf).unwrap();
                            let iter = buf.split_whitespace();
                            iter.map(|x| x.parse().unwrap()).collect()
                            };
    
    let sz = arr.len()-1;

    println!("{:?} ", arr);
    quicksort(&mut arr, 0, sz);
    println!("{:?} ", arr);   
}
