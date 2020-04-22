use std::thread;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut thread_details = Vec::new();
    let counter = Arc::new(Mutex::new(0));
    let hm: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    let input_converted = Arc::new(Mutex::new(input.iter().map(|item| item.to_string()).collect::<Vec<String>>()));
    let res_clone = Arc::clone(&hm);

    for worker in 0..input.len() {
        if worker < worker_count {
            let counter = Arc::clone(&counter);
            let hm_worker = Arc::clone(&hm);
            let input_converted = Arc::clone(&input_converted);

            let t_details = thread::spawn(move || {
                 let mut work_item_index = counter.lock().unwrap();
                  let mut worker_output = hm_worker.lock().unwrap();
                  let strings_converted_copy = input_converted.lock().unwrap();

                  while *work_item_index != strings_converted_copy.len() {
                      *work_item_index += 1;

                      let _map: Vec<_> = strings_converted_copy[*work_item_index - 1].to_lowercase().chars().map(|char| {
                         if char.is_alphabetic() {
                             match worker_output.contains_key(&char) {
                                 true => {
                                     let value = worker_output.get_mut(&char).unwrap();
                                    *value += 1;

                                 }, false => {
                                     worker_output.insert(char, 1 as usize);
                                 }
                             }
                        }
                     }).collect();
                 }
            });

            thread_details.push(t_details);
        }
    }

    for handle in thread_details {
        handle.join().unwrap();
    }

    let result = res_clone.lock().unwrap();
    result.clone()
}
