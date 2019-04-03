
extern crate rand;


use rand::Rng;
use crate::thread_pool::ThreadPool;

pub fn selectionsort<T>(array: &mut [T])
    where T: Ord {
    for i in 0..array.len() {
        let mut lowest_value = i;
        for j in i..array.len() {
            if array[j] < array[lowest_value] {
                lowest_value = j;
            }
        }
        array.swap(i, lowest_value);
    }
}

pub fn inserionsort<T>(array: &mut [T])
    where T: Ord{
    for i in 1..array.len() {
        for j in (0..i).rev() {
            if array[j] > array[j+1] {
                array.swap(j, j+1)
            }
            else {
                break;
            }
        }
    }
}

fn merge<T>(array: &mut [T], mid: usize) -> ()
    where T: Ord + Copy
{
    let length = array.len();
    let mut tmp = vec!{};
    let mut l_count = 0;
    let mut r_count = mid;
    let mut result_count = 0;
    while l_count < mid {
        tmp.push(array[l_count]);
        l_count += 1;
    }
    l_count = 0;
    while  l_count < mid && r_count < length {
        if tmp[l_count] < array[r_count] {
            array[result_count] = tmp[l_count];
            l_count += 1;
        }
        else {
            array[result_count] = array[r_count];
            r_count += 1;
        }
        result_count += 1;
    }
    while l_count < mid {
        array[result_count] = tmp[l_count];
        l_count += 1;
        result_count += 1;
    }
}

fn mergesort_helper<T>(array: &mut [T], length: usize)
    where T: Ord + Copy
{
    if length <= 1 {
        return;
    }
    let mid = length/2;
    mergesort_helper(&mut array[0..mid], mid);
    mergesort_helper(&mut array[mid..length], length-mid);
    merge(array, mid);

}

pub fn mergesort<T>(array: &mut [T])
    where T: Ord + Copy {
    if array.len() == 0 {
        return;
    }
    mergesort_helper(array, array.len());
}

pub fn mergesort_parallel<T>(mut array: &'static mut [T], amount_threads: usize)
    where T: Ord + Copy + Send
{
    if amount_threads == 0 {
        panic!("You need at least one thread!");
    }
    let tpool = ThreadPool::new(amount_threads);
    let segment_size = array.len() / amount_threads;
    let mut disjoint_vec = vec![];

    /* split vectors */
    for _ in 0..amount_threads-1{
        let (left, right) = array.split_at_mut(segment_size);
        disjoint_vec.push(left);
        array = right;
    }
    disjoint_vec.push(array);
    let mut receiver_vector = vec![];
    for part in disjoint_vec {
        receiver_vector.push(
            tpool.exec_with_return_value_nonblocking(
                move || mergesort_helper(part, part.len())
            ));
    }
    let mut disjoint_vec2 = vec![];

    for rx in receiver_vector {
        disjoint_vec2.push(tpool.get_return_value(rx));
    }

    //TODO: merge single parts
    // merge(array, segment_size);


    tpool.kill_thradpool();
    /* reunite vectors */
    // disjoint_vec.fold();

}


#[cfg(test)]
mod tests {

    fn test_empty_vector<T>(f: &Fn(&mut [T]))
        where T: Ord
    {
        let mut empty_vec = [];
        f(&mut empty_vec);
        assert_eq!(empty_vec.len(), 0);
    }

    fn test_sorted_vector(f: &Fn(&mut [u32]))
    {
        let mut empty_vec = [1, 4, 4, 5, 9];
        f(&mut empty_vec);
        assert_eq!(empty_vec, [1, 4, 4, 5, 9]);
    }

    fn test_some_vector(f: &Fn(&mut [u32]))
    {
        let mut empty_vec = [9, 1, 4, 5, 4];
        f(&mut empty_vec);
        assert_eq!(empty_vec, [1, 4, 4, 5, 9]);

        let mut empty_vec = [9,4,3,6,4,2,1];
        f(&mut empty_vec);
        assert_eq!(empty_vec, [1,2,3,4,4,6,9]);

        let mut empty_vec = [11,2,4,101];
        f(&mut empty_vec);
        assert_eq!(empty_vec, [2,4,11,101]);

        let mut empty_vec = [99, 98, 97, 96, 95, 94, 93, 92, 91, 90];
        f(&mut empty_vec);
        assert_eq!(empty_vec, [90,91,92,93,94,95,96,97,98,99]);
    }

    fn test_random_vector(f: &Fn(&mut [u32]))
    {
        let mut rng = super::rand::thread_rng();

        let mut vec: [u32; 1000] = unsafe { std::mem::uninitialized() };
        for a in 0..vec.len()-1 {
            vec[a] = crate::sort::rand::Rng::gen(&mut rng);
        }
        f(&mut vec);
        //assert_eq!(vec, );
    }

    fn test_all_with_u32(f: &Fn(&mut [u32]))
    {
        test_empty_vector(f);
        test_sorted_vector(f);
        test_some_vector(f);
        test_random_vector(f);
    }

    #[test]
    fn test_selectionsort() {
        test_all_with_u32(&super::selectionsort);
    }

    #[test]
    fn test_insertionsort() {
        test_all_with_u32(&super::inserionsort);
    }

    #[test]
    fn test_mergesort() {
        test_all_with_u32(&super::mergesort);
    }

}
