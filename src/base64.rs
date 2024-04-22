// //二分法查找
// fn binary_search<T: Ord>(a: &[T], value: T) -> Option<usize> {
//     // Initialize the low and high pointers
//     let mut low = 0;
//     let mut high = a.len();

//     // Perform the binary search
//     while low < high {
//         // Calculate the middle index
//         let mid = (low + high) / 2;
//         // If the middle value is less than the target, move the low pointer up
//         if a[mid] < value {
//             low = mid + 1;
//             // If the middle value is equal to the target, return the middle index
//         } else {
//             // If the middle value is greater than the target, move the high pointer down
//             high = mid;
//         }
//     }
//     // If the low pointer is less than the array length and the value at the low pointer is equal to the target, return the low pointer
//     if low < a.len() && a[low] == value {
//         Some(low)
//     // Otherwise, return None
//     } else {
//         None
//     }
// }

// process_csv
