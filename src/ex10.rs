use std::fmt;

#[derive(Debug, PartialEq, PartialOrd)] 
pub struct Clock {
    h : i32,
    m : i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut h = (hours % 24);
        if (h < 0) {
            h += 24;
        }
        h *= 60;
        // println!("{}", h / 60);
        let mut m = minutes;

        let mut t = h + m;

        h = t / 60;
        m = t - (h * 60);

        while (h >= 24) {
            h -= 24;
        }
            
        return Clock{h, m};
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut t = (self.h * 60) + self.m;
        t += minutes;
        
        let mut h = t / 60;
        let mut m = t - (h * 60);

        while (h >= 24) {
            h -= 24;
        }

        return Clock{h, m};
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.h, self.m)
    }
}