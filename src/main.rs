#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate icalendar;
extern crate chrono;
use std::ops::Add;
use chrono::{Utc, TimeZone};
use icalendar::{Component, Calendar, Event};

#[get("/")]
fn hello() -> &'static str {
    return "Hello, world!"
}

#[get("/lily")]
fn lily() -> String {
    return gen_cal().to_string()
}

fn main() {
    rocket::ignite().mount("/",
        routes![hello, lily]
    ).launch();
}

fn gen_cal() -> icalendar::Calendar {
    let mut calendar = Calendar::new();

    for x in 0..52 {
        calendar.push(Event::new()
            .all_day(Utc.ymd(2017, 4, 12).add(chrono::Duration::weeks(x)))
            .summary(&format!("Lily is {} weeks old", x))
            .done());
    }

    return calendar;
}
