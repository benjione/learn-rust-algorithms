extern crate rand;


use rand::Rng;
use std::thread;

//
// fn mergesort_helper<'a, T>(array: &'static mut [T], length: usize)
//     where T: Ord + Copy + std::marker::Send
// {
//     if length <= 1 {
//         return;
//     }
//     let mid = length/2;
//     let t1 = thread::spawn(move || mergesort_helper(&mut array[0..mid], mid));
//     let t2 = thread::spawn(move || mergesort_helper(&mut array[mid..length], length-mid));
//     t1.join();
//     t2.join();
//     let mut tmp = vec!{};
//     let mut l_count = 0;
//     let mut r_count = mid;
//     let mut result_count = 0;
//     while l_count < mid {
//         tmp.push(array[l_count]);
//         l_count += 1;
//     }
//     l_count = 0;
//     while  l_count < mid && r_count < length {
//         if tmp[l_count] < array[r_count] {
//             array[result_count] = tmp[l_count];
//             l_count += 1;
//         }
//         else {
//             array[result_count] = array[r_count];
//             r_count += 1;
//         }
//         result_count += 1;
//     }
//     while l_count < mid {
//         array[result_count] = tmp[l_count];
//         l_count += 1;
//         result_count += 1;
//     }
//
// }
//
// pub fn mergesort<T>(array: &'static mut [T])
//     where T: Ord + Copy + std::marker::Send {
//     if array.len() == 0 {
//         return;
//     }
//     mergesort_helper(array, array.len());
// }

pub fn map_parallel() -> () {
    
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
    fn test_mergesort() {
        test_all_with_u32(&super::mergesort);
    }

}
