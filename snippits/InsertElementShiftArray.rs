fn insert(x: i32, arr: &mut [i32; 3]) {
    // Skip over lesser elements:
    let mut i = 0;
    while i < (arr.len() - 1) && arr[i] < x  { 
        i += 1; 
    }
    // Shift elements down:
    for j in 0..i { 
        arr[j] = arr[j + 1]; 
    }
    // Insert x at i'th element:
    arr[i] = x;
}
