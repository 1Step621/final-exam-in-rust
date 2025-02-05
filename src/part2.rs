use std::{
    fmt::Display,
    fs,
    io::stdin,
    ops::{Deref, DerefMut},
};

use itertools::Itertools;

#[derive(Clone, Debug)]
struct Student {
    name: String,
    score: u8,
}

impl From<&str> for Student {
    fn from(s: &str) -> Self {
        let s = s.split(" ").collect::<Vec<_>>();
        Student {
            name: s[0].to_string(),
            score: s[1].parse().unwrap(),
        }
    }
}

impl Display for Student {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.score)
    }
}

impl From<&Student> for String {
    fn from(s: &Student) -> Self {
        format!("{} {}", s.score, s.name)
    }
}

#[derive(Clone, Debug)]
struct VecStudent(Vec<Student>);

impl Deref for VecStudent {
    type Target = Vec<Student>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VecStudent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Student>> for VecStudent {
    fn from(v: Vec<Student>) -> Self {
        Self(v)
    }
}

impl Display for VecStudent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.iter().map(|s| format!("{}", s)).join("\n"))?;
        Ok(())
    }
}

impl From<VecStudent> for String {
    fn from(v: VecStudent) -> Self {
        v.iter().map(String::from).join("\n")
    }
}

impl FromIterator<Student> for VecStudent {
    fn from_iter<I: IntoIterator<Item = Student>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

pub fn main() {
    let stdin = stdin();

    let mut students = fs::read_to_string("students.txt")
        .unwrap()
        .lines()
        .map(Student::from)
        .collect::<VecStudent>();

    println!("Average score: {}", {
        let sum = students.iter().map(|s| s.score as f64).sum::<f64>();
        sum / students.len() as f64
    });

    println!();

    println!("Students sorted by score:");
    students.sort_by_key(|s| s.score);
    students.reverse();
    println!("{}", students);

    println!();

    println!("Students sorted by name:");
    students.sort_by_key(|s| s.name.clone());
    println!("{}", students);

    println!();

    println!("Find student by name:");
    let mut name = String::new();
    stdin.read_line(&mut name).unwrap();
    let name = name.trim();
    let student = students.iter().find(|s| s.name == name);
    match student {
        Some(student) => println!("{}", student),
        None => println!("Student not found"),
    }

    println!();

    let content = students
        .iter().filter(|&s| s.score >= 80).cloned()
        .collect::<VecStudent>();
    fs::write("filtered_students.txt", String::from(content)).unwrap();
    println!("Students with score above 80 were output to filtered_students.txt");

    println!();

    let new_students = fs::read_to_string("new_students.txt")
        .unwrap()
        .lines()
        .map(Student::from)
        .collect::<VecStudent>();
    students.extend(new_students.iter().cloned());
    println!("Students after adding new students:");
    println!("{}", students);
}
