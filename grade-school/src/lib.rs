use std::collections::HashMap;

pub struct School {
    ledger: HashMap<u32, Vec<String>>
}

impl School {
    pub fn new() -> School {
        School {
            ledger: HashMap::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let _option = match self.ledger.contains_key(&grade) {
            true => {
                let grade_vec = self.ledger.get_mut(&grade).unwrap();
                grade_vec.push(student.to_string());
                grade_vec.sort();
            },
            false => {
                let mut vector_students = Vec::new();
                vector_students.push(student.to_string());
                self.ledger.insert(grade, vector_students);
            }
        };
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut keys: Vec<u32> = self.ledger.keys().cloned().collect();
        keys.sort();

        keys
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.ledger.get(&grade).cloned() 
    }
}
