
enum BookingError {
    NoVacancies,
    BadCourseName {course: String},
    PaymentError {course: String, expected: f64},
}

fn book_course(course: &str, payment: f64)
    -> Result<i32, BookingError> {

        if course == "Rust 202" {
            return Err(BookingError::NoVacancies)
        }

        if course != "Rust 101" {
            return Err(BookingError::BadCourseName
                {course: String::from(course)});
        }

        if (payment as i32) < 100 {
            return Err(BookingError::PaymentError 
                {course: String::from(course), expected: 100.0})
        }

        Ok(42)
        
    }



fn decode(t: (i32, i32, i32)) -> i32 {
    let ((1, x, 4) | (x, _, _)) = t;
    x
}

fn report_error(result: Result<i32, BookingError>)
    -> Option<i32> {

        match result {
            Err(BookingError::NoVacancies)
                => println!("That course is full"),
            
            Err(BookingError::BadCourseName
                {course: course_name})
                => println!("The course '{0}' does not exists",
                   course_name),
            
            Err(BookingError::PaymentError {course, expected})
                => println!("{0} costs at least ${1}",
                course, expected),
            
            Ok(reference) => return Some(reference),
        }
        return None;
    }

fn main() {
    let a = (1,2,4);
    let b = (3,2,5);

    let x = decode(a);
    let y = decode(b);

    println!("{a:?} -> {x} and {b:?} -> {y}");

    println!("---");

    // Your booking reference is 42
    let Some(reference) =
        report_error(book_course("Rust 101", 100.00)) else {
            return;
        };
        println!("Your booking reference is {reference}");

    // That course is full
    let Some(reference) =
        report_error(book_course("Rust 202", 100.00)) else {
            return;
        };
        println!("Your booking reference is {reference}");

    // no output
    let Some(reference) =
        report_error(book_course("Rust 303", 100.00)) else {
            return;
        };
        println!("Your booking reference is {reference}");

    // no output
    let Some(reference) =
        report_error(book_course("Rust 101", 50.00)) else {
            return;
        };
        println!("Your booking reference is {reference}");

    // match book_course("Rust 101", 100.0) {
    //     Ok(booking_reference)
    //         => println!("Course booked. Reference {0}",
    //             booking_reference),
        
    //     Err(BookingError::NoVacancies)
    //         => println!("That course is full"),

    //     Err(BookingError::BadCourseName 
    //         {course: course_name})
    //         => println!("The course '{0}' does not exist",
    //             course_name),
            
    //     Err(BookingError::PaymentError 
    //         {course, expected})
    //         => println!("{0} costs of at least ${1}",
    //             course, expected)

    // }
    

    // match book_course("Rust 202", 100.0) {
    //     Ok(booking_reference)
    //         => println!("Course booked. Reference {0}",
    //             booking_reference),
        
    //     Err(BookingError::NoVacancies)
    //         => println!("That course is full"),

    //     Err(BookingError::BadCourseName 
    //         {course: course_name})
    //         => println!("The course '{0}' does not exist",
    //             course_name),
            
    //     Err(BookingError::PaymentError 
    //         {course, expected})
    //         => println!("{0} costs of at least ${1}",
    //             course, expected)

    // }


    // match book_course("Rust 303", 100.0) {
    //     Ok(booking_reference)
    //         => println!("Course booked. Reference {0}",
    //             booking_reference),
        
    //     Err(BookingError::NoVacancies)
    //         => println!("That course is full"),

    //     Err(BookingError::BadCourseName 
    //         {course: course_name})
    //         => println!("The course '{0}' does not exist",
    //             course_name),
            
    //     Err(BookingError::PaymentError 
    //         {course, expected})
    //         => println!("{0} costs of at least ${1}",
    //             course, expected)

    // }


    // match book_course("Rust 101", 50.0) {
    //     Ok(booking_reference)
    //         => println!("Course booked. Reference {0}",
    //             booking_reference),
        
    //     Err(BookingError::NoVacancies)
    //         => println!("That course is full"),

    //     Err(BookingError::BadCourseName 
    //         {course: course_name})
    //         => println!("The course '{0}' does not exist",
    //             course_name),
            
    //     Err(BookingError::PaymentError 
    //         {course, expected})
    //         => println!("{0} costs of at least ${1}",
    //             course, expected)

    // }
}