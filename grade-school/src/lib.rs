use std::collections::HashMap;

pub struct School {
    enrolled: HashMap<u32, Vec<String>>,
}

impl Default for School {
    fn default() -> Self {
        Self::new()
    }
}

impl School {
    pub fn new() -> School {
        School {
            enrolled: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let kids = self.enrolled.entry(grade).or_insert_with(Vec::new);
        kids.push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<_> = self.enrolled.keys().copied().collect();
        grades.sort();
        grades
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        let gr = self.enrolled.get(&grade);
        match gr {
            Some(gr) => {
                let mut gr = gr.clone();
                gr.sort();
                Some(gr)
            }
            None => None,
        }
    }
}
