#[derive(Debug)]
struct Date {
    year: i16,
    month: u8,
    day: u8,
}

impl Date {
    fn years_before(&self, n: i16) -> Date {
        Date {
            year: self.year - n,
            ..*self
        }
    }

    fn is_before(&self, other: &Date) -> bool {
        self.year < other.year ||
            (self.year == other.year &&
                (self.month < other.month ||
                    (self.month == other.month && 
                        self.day < other.day
                    )
                )
            )
    }
}

struct Student {
    date_of_birth: Date,
    date_of_graduation: Date,
    name: String,
}

impl Student {
    fn age_on(&self, date: Date) -> i16 {
        let year_age = date.year - self.date_of_birth.year;
        let birthday = Date {
            year: date.year,
            ..self.date_of_birth};
        
        if date.is_before(&birthday) {
            return year_age - 1;
        }
        year_age
    }
}

fn describe(pupil: &Student, course_length: i16) {
    let induction_year = pupil.date_of_graduation
        .years_before(course_length)
        .year;

    let induction_date = Date {
        day: 1,
        month: 9,
        year: induction_year
    };
    let age_on_induction = pupil.age_on(induction_date);

    println!("{0} will be inducted in {1} at {2} years old.",
        pupil.name, induction_year, age_on_induction);
    
}

fn main() {
    let pupil1 = Student {
        date_of_birth: Date {year: 2029, month: 5, day: 12},
        date_of_graduation: Date{year: 2050, month: 7, day: 4},
        name: String::from("Andy"),
    };

    let pupil2 = Student {
        date_of_birth: Date {year: 2029, month: 10, day: 20},
        date_of_graduation: Date{year: 2050, month: 7, day: 4},
        name: String::from("Beth"),
    };

    let course_length = 3;

    describe(&pupil1, course_length);
    describe(&pupil2, course_length);

    println!("{0} will graduate on {1:?}",
        pupil1.name,
        pupil1.date_of_graduation);
    
}
