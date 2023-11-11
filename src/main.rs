use std::collections::HashMap;

// 学生结构体信息，包括学生的
#[derive(Debug)]
struct Student {
    name: String,
    student_id: u32,
    club: Option<Club>,
    class: Option<Class>,
    courses: Vec<Course>,
}

// 定义社团的枚举类型
#[derive(Debug)]
enum Club {
    Basketball,
    Robot,
}

// 定义班级的枚举类型
#[derive(Debug)]
enum Class {
    Class1,
    Class2,
}

// 定义课程的枚举类型
#[derive(Debug)]
enum Course {
    Maths,
    Englis,
    Physics,
}

impl Student {
    // 根据学生学号和名字创建学生实例
    fn new(name: String, student_id: u32) -> Self {
        Self {
            student_id,
            name,
            club: None,
            class: None,
            courses: Vec::new(),
        }
    }

    // 添加社团
    fn add_club(&mut self, club: Club) {
        self.club = Some(club);
    }

    // 添加班级
    fn add_class(&mut self, class: Class) {
        self.class = Some(class);
    }

    // 添加课程
    fn add_course(&mut self, course: Vec<Course>) {
        self.courses.extend(course);
    }
}

// 定义学生管理系统
struct StudentManagementSystem {
    students: HashMap<String, Student>,
}

impl StudentManagementSystem {
    // 启动学生管理系统
    fn set_up() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    // 打印学生所有信息
    fn print_students(&self) {
        for (name, student) in &self.students {
            println!("{}: {:?}", name, student);
        }
    }

    // 添加一个学生
    fn add_student(&mut self, student_id: u32, name: &str) {
        let student = Student::new(name.to_string(), student_id);
        self.students.insert(name.to_string(), student);
    }

    // 查找一个学生信息
    fn find_student(&self, name: &str) {
        match self.students.get(name) {
            Some(student) => println!("Find the information of {}: {:?}", name, student),
            None => println!("The student is not in the system."),
        }
    }

    // Anchor start: update the information of student
    // 更新学生信息

    // 修改学生学号
    fn update_student_id(&mut self, name: &str, new_student_id: u32) {
        if let Some(student) = self.students.get_mut(name) {
            student.student_id = new_student_id;
            println!("update correctly: {:?}", student);
        }
    }

    // 更新学生所属社团
    fn update_student_club(&mut self, name: &str, club: Club) {
        if let Some(student) = self.students.get_mut(name) {
            student.add_club(club);
            println!("update correctly: {:?}", student);
        }
    }

    // 更新学生所属班级
    fn update_student_class(&mut self, name: &str, class: Class) {
        if let Some(student) = self.students.get_mut(name) {
            student.add_class(class);
            println!("update correctly: {:?}", student);
        }
    }

    // 更新学生的课程
    fn update_student_course(&mut self, name: &str, course: Vec<Course>) {
        if let Some(student) = self.students.get_mut(name) {
            student.add_course(course);
            println!("update correctly: {:?}", student);
        }
    }

    // Anchor end: update the information of student

    // 删除一个学生
    fn delete_student(&mut self, name: &str) {
        if self.students.contains_key(name) {
            println!("{} remove from the system.", name);
            self.students.remove(name);
        } else {
            println!("The student is not in the system.");
        }
    }
}

fn main() {
    // 启动学生管理系统
    let mut sms = StudentManagementSystem::set_up();

    // 添加学生
    sms.add_student(1, "Alice");
    sms.add_student(2, "Bob");

    // 查找学生
    sms.find_student("Alice");

    // start: 更新学生信息
    // 更新学生学号
    sms.update_student_id("Alice", 3);

    // 更新学生所属社团
    sms.update_student_club("Alice", Club::Robot);

    // 更新学生所属班级
    sms.update_student_class("Alice", Class::Class1);

    // 更新学生的课程
    sms.update_student_course("Alice", vec![Course::Maths, Course::Englis]);

    // end: 更新学生信息

    // 删除学生
    sms.delete_student("Bob");

    // 打印学生管理系统中的学生
    sms.print_students()
}
