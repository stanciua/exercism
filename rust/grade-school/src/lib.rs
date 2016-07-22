use std::collections::HashMap;

#[derive(Default)]
pub struct School {
  grades: HashMap<i32, Vec<String>>,
}

impl<'a> School {
  pub fn new() -> School {
    School { grades: HashMap::new() }
  }

  pub fn add(&mut self, grade: i32, name: &str) {
    {
      let mut slot = self.grades.entry(grade).or_insert_with(Vec::new);
      slot.push(name.to_owned());
    }
    self.grades.get_mut(&grade).unwrap_or(&mut Vec::<String>::new()).sort_by(|a, b| a.cmp(b));
  } 

  pub fn grades(&self) -> Vec<i32> {
    let mut v = self.grades.iter().map(|(grade, _)| *grade).collect::<Vec<_>>();
    v.sort_by(|a, b| a.cmp(b));
    v
  }

  pub fn grade(&'a self, grade: i32) -> Option<&'a Vec<String>> {    
    self.grades.get(&grade) 
  }
}
