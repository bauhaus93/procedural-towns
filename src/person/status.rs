use crate::utility::Date;

pub struct Status {
    start: Date,
    stop: Option<Date>,
    stat_type: StatusType
}

enum StatusType {
    Pregnancy(u32),
    Dead
}
