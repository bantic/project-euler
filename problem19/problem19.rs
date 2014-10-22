use std::fmt;

#[deriving(PartialEq)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}
impl Day {
    fn next(&self) -> Day {
        match *self {
            Monday => Tuesday,
            Tuesday => Wednesday,
            Wednesday => Thursday,
            Thursday => Friday,
            Friday => Saturday,
            Saturday => Sunday,
            Sunday   => Monday
        }
    }
}

#[deriving(PartialEq)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December
}

impl Month {
    fn next(&self) -> Month {
        match *self {
            January => February,
            February => March,
            March => April,
            April => May,
            May => June,
            June => July,
            July => August,
            August => September,
            September => October,
            October => November,
            November => December,
            December => January
        }
    }

    fn days(&self, leap:bool) -> uint {
        match *self {
            June => 30u,
            April => 30u,
            September => 30u,
            November => 30u,
            February => if leap {
                29u
            } else {
                28u
            },
            _ => 31u
        }
    }
}

struct Date {
    year: uint,
    day_of_month: uint,
    day: Day,
    month: Month
}

impl Date {
    fn new(day:Day, day_of_month:uint, month:Month, year:uint) -> Date {
        Date { year:year, day_of_month: day_of_month, day:day, month: month }
    }

    fn eq(&self, other:Date) -> bool {
        self.day_of_month == other.day_of_month &&
            self.month == other.month &&
            self.year == other.year
    }

    fn inc(&mut self) {
        let next_day_of_month = self.day_of_month + 1;

        if next_day_of_month > self.month.days(self.is_leap_year()) {
            self.day_of_month = 1;
            self.month = self.month.next();
            if self.month == January {
                self.year = self.year + 1;
            }
        } else {
            self.day_of_month = next_day_of_month;
        }

        self.day = self.day.next();
    }

    fn is_leap_year(&self) -> bool {
        if self.year % 4 == 0 {
            if self.year % 100 == 0 {
                self.year % 400 == 0
            } else {
                true
            }
        } else {
            false
        }
    }
}

impl fmt::Show for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {} {} {}", self.day, self.day_of_month, self.month, self.year)
    }
}


impl fmt::Show for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match *self {
            Monday => "Monday",
            Tuesday => "Tuesday",
            Wednesday => "Wednesday",
            Thursday => "Thursday",
            Friday => "Friday",
            Saturday => "Saturday",
            Sunday => "Sunday"
        };
        write!(f, "{}", str)
    }
}

impl fmt::Show for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match *self {
            January => "January",
            February => "February",
            March => "March",
            April => "April",
            May => "May",
            June => "June",
            July => "July",
            August => "August",
            September => "September",
            October => "October",
            November => "November",
            December => "December"
        };
        write!(f, "{}", str)
    }
}

fn main() {
    let mut d = Date::new(Monday, 1, January, 1900);
    let mut do_count = false;
    let mut count = 0u;

    // the day is probably wrong but it doesn't matter, we just check month,year,day_of_month for equality
    let count_after = Date::new(Monday, 1, January, 1901);
    // the day is probably wrong but it doesn't matter, we just check month,year,day_of_month for equality
    let stop_after = Date::new(Monday, 31, December, 2000);

    loop {
        d.inc();
        if d.eq(count_after) {
            do_count = true;
            println!("Starting to check!");
        }

        if do_count && d.day == Sunday && d.day_of_month == 1{
            count += 1;
        }

        if d.eq(stop_after) {
            break;
        }
    }

    println!("Count: {}",count);
}
