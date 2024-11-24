use std::thread;
use std::collections::HashMap;
use std::collections::BTreeMap;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let data = "result";
    // Number of threads to use
    let num_threads = 4;

    // Total numbers to count
    let total_count = 100;

    // Calculate the range each thread will handle
    let range_per_thread = total_count / num_threads;

    // Create a vector to hold the thread handles
    let mut handles = vec![];

    for i in 0..num_threads {
        let start = i * range_per_thread + 1;
        // let end = if i == num_threads - 1 {
        //     total_count
        // } else {
        //     start + range_per_thread - 1
        // };
        let end;
        if i == num_threads -1 {
            end = total_count
        } else {
            end = add(start, range_per_thread - 1)
        }
        let value = format!("{} start:{} end:{}", data, start, end);
        println!("{}", value);

        // Spawn a new thread for each range
        let handle = thread::spawn(move || {
            for num in start..=end {
                println!("Thread {:?} counting: {}", i + 1, num);
            }
        });

        // Push the thread handle to the vector
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads have finished counting.");

    let mut map1: HashMap<&str, [i32; 3]> = HashMap::new();
    let arr = [10, 20, 30];
    map1.insert("first", [1, 2, 3]);
    map1.insert("second", arr);
    // if let Some(arr) = map1.get("first") {
    //     println!("map1 ==> Value for 'first': {:?}", arr); // Output: [1, 2, 3]
    // }
    for (key, value) in &map1 {
        println!("map1 ==> Key: {}, Value: {:?}", key, value);
    }
    println!();
    
    let mut map2: HashMap<&str, Vec<i32>> = HashMap::new();
    let mut vec = vec![10, 20, 30];
    vec.push(40); // Add an element
    map2.insert("1st", vec![1, 2, 3]);
    map2.insert("2nd", vec.clone());
    map2.insert("3rd", vec.clone());
    map2.insert("4th", vec);

    // Add more elements to an existing key's value
    if let Some(vec) = map2.get_mut("first") {
        vec.push(100); // Add 4 to the vector
    }
    // if let Some(vec) = map2.get("first") {
    //     println!("map2 ==> Value for 'first': {:?}", vec); // Output: [1, 2, 3, 4]
    // }
    for (key, value) in &map2 {
        println!("map2 ==> Key: {}, Value: {:?}", key, value);
    }
    println!();

    let mut map3: BTreeMap<&str, Vec<i32>> = BTreeMap::new();
    let mut vec = vec![10, 20, 30];
    vec.push(40); // Add an element
    map3.insert("1st", vec![1, 2, 3]);
    map3.insert("2nd", vec.clone());
    map3.insert("3rd", vec.clone());
    map3.insert("4th", vec);

    // Add more elements to an existing key's value
    if let Some(vec) = map3.get_mut("first") {
        vec.push(100); // Add 4 to the vector
    }

    for (key, value) in &map3 {
        println!("map3 ==> Key: {}, Value: {:?}", key, value);
    }
}
