use std::rc::Rc;
use std::cell::RefCell;

// Redesign this stuff to not ruin this
// To move around this issue create a data set consisting of
// students
// courses
// Enrollment which connects a student id with a course id.

pub fn glob() {
    println!("----------- circ_ref.rs -----------");
    demo();
}

// struct Student<'a> {
//     name: String,
//     course: Vec<&'a Course<'a>>
// }
//
// impl<'a> Student<'a> {
//     fn new(name: &str) -> Student<'a> {
//         return Student {
//             name: name.into(),
//             course: Vec::new()
//         }
//     }
// }
//
// struct Course<'a> {
//     name: String,
//     students: Vec<&'a Student<'a>>
// }
//
// impl<'a> Course<'a> {
//     fn new(name: &str) -> Course<'a> {
//         return Course {
//             name: name.into(),
//             students: Vec::new()
//         }
//     }
//
//     fn add_student(&'a mut self, student: &'a mut Student<'a>) {
//         student.course.push(self);
//         self.students.push(student);
//         // RefCell
//     }
// }

struct Student {
    name: String,
    course: Vec<Rc<RefCell<Course>>>
}

impl Student {
    fn new(name: &str) -> Student {
        return Student {
            name: name.into(),
            course: Vec::new()
        }
    }
}

struct Course {
    name: String,
    students: Vec<Rc<RefCell<Student>>>
}

impl Course{
    fn new(name: &str) -> Course {
        return Course {
            name: name.into(),
            students: Vec::new()
        }
    }

    fn add_student(
        course: Rc<RefCell<Course>>,
        student: Rc<RefCell<Student>>
    ) {
        student.borrow_mut().course.push(course.clone());
        course.borrow_mut().students.push(student);
    }
}

fn demo() {
    let john = Rc::new(
        RefCell::new(
            Student::new("John")));

    let course = Course::new("Rust Course");
    let magic_course = Rc::new(RefCell::new(course));

    // course.add_student(john); // Problem of lifetimes -> Rc
    Course::add_student(magic_course.clone(), john);
}

