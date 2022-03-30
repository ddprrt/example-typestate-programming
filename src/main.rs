use std::marker::PhantomData;

struct TwelveHourClock;
struct TwentyFourHourClock;

#[derive(Debug)]
struct Clock<Kind = TwentyFourHourClock> {
    hours: i32,
    minutes: i32,
    kind: PhantomData<Kind>,
}

impl Clock {
    fn new(hours: i32, minutes: i32) -> Self {
        Self {
            hours,
            minutes,
            kind: PhantomData,
        }
        .normalize()
    }

    fn normalize(&mut self) -> Self {
        let mut hours = (self.hours + self.minutes / 60) % 24;
        let mut minutes = self.minutes % 60;

        if minutes < 0 {
            minutes += 60;
            hours -= 1;
        }

        if hours < 0 {
            hours += 24;
        }

        Self {
            hours,
            minutes,
            kind: PhantomData,
        }
    }

    fn add_minutes(&mut self, min: i32) -> Self {
        Self {
            hours: self.hours,
            minutes: self.minutes + min,
            kind: PhantomData
        }.normalize()
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            hours: Default::default(),
            minutes: Default::default(),
            kind: PhantomData,
        }
    }
}

impl std::fmt::Display for Clock<TwentyFourHourClock> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl std::fmt::Display for Clock<TwelveHourClock> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (hours, time_code) = match self.hours {
            0..=11 => (self.hours, "a.m."),
            12 => (self.hours, "p.m."),
            _ => (self.hours - 12, "p.m.")
        };
        write!(f, "{:0>2}:{:0>2} {}", hours, self.minutes, time_code)
    }
}

impl Clock<TwentyFourHourClock> {
    fn as_twelve_hour_clock(&self) -> Clock<TwelveHourClock> {
        Clock {
            hours: self.hours,
            minutes: self.minutes,
            kind: PhantomData,
        }
    }
}

impl Clock<TwelveHourClock> {
    fn as_twenty_four_hour_clock(&self) -> Clock<TwentyFourHourClock> {
        Clock {
            hours: self.hours,
            minutes: self.minutes,
            kind: PhantomData,
        }
    }
}

impl std::ops::Add for Clock {
    type Output = Clock;

    fn add(self, rhs: Self) -> Self::Output {
        Clock::new(self.hours + rhs.hours, self.minutes + rhs.minutes)
    }
}

impl std::ops::Add<i32> for Clock {
    type Output = Clock;

    fn add(self, rhs: i32) -> Self::Output {
        Clock::new(self.hours, self.minutes + rhs)
    }
}

impl From<i32> for Clock {
    fn from(val: i32) -> Clock {
        Clock::new(0, val)
    }
}

fn main() {
    let clock = Clock::new(14, 55);
    let clock_2 = Clock::new(34, 155);
    let default_clock: Clock = Default::default();

    let clock = clock + clock_2;

    println!("{}", clock);

    let clock = default_clock + 1055;

    println!("{}", clock.as_twelve_hour_clock());

    let clock: Clock = 2055.into();

    println!("{}", clock);
}
