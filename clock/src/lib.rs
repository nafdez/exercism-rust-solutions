use std::fmt;

pub struct Clock {
   minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
       Clock { minutes: (minutes + (hours * 60)) }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
       Clock { minutes: (self.minutes + minutes) }
    }

    fn to_hours_and_minutes(&self) -> (i32, i32) {
        let hours: i32 = ((((self.minutes as f32 / 60f32).floor() % 24f32).floor() + 24f32) % 24f32).floor() as i32;
        let minutes: i32 = (((self.minutes as f32 % 60f32).floor() + 60f32) % 60f32).floor() as i32;

        (hours, minutes)
    }

    pub fn to_string(&self) -> String {
        let (hours, minutes) = self.to_hours_and_minutes();
        format!("{:02}:{:02}", hours, minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
