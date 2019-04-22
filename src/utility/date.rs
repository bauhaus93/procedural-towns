use std::cmp::Ordering;
use std::fmt;
use std::ops::{ Add, AddAssign, Sub, SubAssign };
use rand::Rng;

// 1 year has 12 months with 30 days
const DAYS_PER_MONTH: u32 = 30;
const MONTHS_PER_YEAR: u32 = 12;
const DAYS_PER_YEAR: u32 = DAYS_PER_MONTH * MONTHS_PER_YEAR;

#[derive(Eq, Clone, Copy)]
pub struct Date {
    day: u32,
    month: u32,
    year: u32
}

impl Date {

    pub fn get_day(&self) -> u32 {
        self.day
    }
    pub fn get_month(&self) -> u32 {
        self.month
    }
    pub fn get_year(&self) -> u32 {
        self.year
    }


    pub fn random<R: Rng + ?Sized>(min_year: u32, max_year: u32, rng: &mut R) -> Date {
        Self {
            day: rng.gen_range(0, DAYS_PER_MONTH),
            month: rng.gen_range(0, MONTHS_PER_YEAR),
            year: rng.gen_range(min_year, max_year)
        }
    }

    pub fn random_future_years_range<R: Rng + ?Sized>(&self, range: (u32, u32), rng: &mut R) -> Date {
        *self + rng.gen_range(range.0 * DAYS_PER_YEAR, range.1 * DAYS_PER_YEAR)
    }

    pub fn random_past_years_range<R: Rng + ?Sized>(&self, range: (u32, u32), rng: &mut R) -> Date {
        *self - rng.gen_range(range.0 * DAYS_PER_YEAR, range.1 * DAYS_PER_YEAR)
    }
    
    pub fn set_day(&mut self, day: u32) {
        debug_assert!(day < DAYS_PER_MONTH);
        self.day = day;
    }
    pub fn set_month(&mut self, month: u32) {
        debug_assert!(month < MONTHS_PER_YEAR);
        self.month = month;
    }
    pub fn set_year(&mut self, year: u32) {
        self.year = year;
    }

    pub fn advance_years(&mut self, years: u32) {
        self.year += years;
    }
}

impl Default for Date {
    fn default() -> Self {
        Self {
            day: 0,
            month: 0,
            year: 1000
        }
    }
}

impl Add<u32> for Date {
    type Output = Date;

    fn add(mut self, day_amount: u32) -> Self {
        if day_amount >= DAYS_PER_YEAR {
            self.year += day_amount / DAYS_PER_YEAR;
        }
        let days_left = day_amount % DAYS_PER_YEAR;
        let additional_months = days_left / DAYS_PER_MONTH;
        let additional_days = days_left % DAYS_PER_MONTH;
        if additional_months > 0 {
            if self.month + additional_months < MONTHS_PER_YEAR {
                self.month += additional_months;
            } else {
                self.month = (self.month + additional_months) % MONTHS_PER_YEAR;
                self.year += 1;
            }
        }
        if additional_days > 0 {
            if self.day + additional_days < DAYS_PER_MONTH {
                self.day += additional_days;
            } else {
                self.day = (self.day + additional_days) % DAYS_PER_MONTH;
                self.month += 1;
                if self.month == MONTHS_PER_YEAR {
                    self.month = 0;
                    self.year += 1;
                }
            }
        }
        self
    }
}

impl Sub<u32> for Date {
    type Output = Date;

    fn sub(mut self, day_amount: u32) -> Self {
        if day_amount >= DAYS_PER_YEAR {
            self.year -= day_amount / DAYS_PER_YEAR;
        }
        let days_left = day_amount % DAYS_PER_YEAR;
        let additional_months = days_left / DAYS_PER_MONTH;
        let additional_days = days_left % DAYS_PER_MONTH;
        if additional_months > 0 {
            if self.month >= additional_months {
                self.month -= additional_months;
            } else {
                let diff = (self.month as i32 - additional_months as i32).abs() as u32;
                self.month = MONTHS_PER_YEAR - diff;
                self.year -= 1;
            }
        }
        if additional_days > 0 {
            if self.day >= additional_days {
                self.day -= additional_days;
            } else {
                let diff = (self.day as i32 - additional_days as i32).abs() as u32;
                self.day = DAYS_PER_MONTH - diff;
                if self.month > 0 {
                    self.month -= 1;
                } else {
                    self.month = MONTHS_PER_YEAR - 1;
                    self.year -= 1;
                }
            }
        }
        self
    }
}

impl AddAssign<u32> for Date {
    fn add_assign(&mut self, day_amount: u32) {
        *self = *self + day_amount;
    }
}

impl SubAssign<u32> for Date {
    fn sub_assign(&mut self, day_amount: u32) {
        *self = *self - day_amount;
    }
}


impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.year < other.year {
            Ordering::Less
        } else if self.year > other.year {
            Ordering::Greater
        } else {
            if self.month < other.month {
                Ordering::Less
            } else if self.month > other.month {
                Ordering::Greater
            } else {
                if self.day < other.day {
                    Ordering::Less
                } else if self.day > other.day {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        }
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day &&
        self.month == other.month &&
        self.year == other.year
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month + 1, self.day + 1)
    }
}