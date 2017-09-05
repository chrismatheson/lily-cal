#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate icalendar;
extern crate chrono;
extern crate domafic;
use domafic::tags::{div, h1, p};
use std::marker::PhantomData;
use chrono::prelude::*;
use icalendar::{Component, Calendar, Event, Property};
use rocket::response::{Failure};
use rocket::http::Status;
use std::ops::Add;

type Msg = ();

#[get("/")]
fn index() -> String {
    return div((
        h1("Welcome to Baby-Cal"),
        p("This page will create you a calendar with usefull at-a-glance info about your babies age."),
        PhantomData::<Msg>
    )).to_string();
}


#[get("/lily")]
fn lily() -> Result<String, Failure> {
    return cal("Lily".to_owned(), "2017-04-12T00:00:00+00:00".to_owned());
}

#[get("/cal/<name>/<dob>")]
fn cal(name: String, dob: String) -> Result<String, Failure> {
    return match DateTime::parse_from_rfc3339(dob.as_str()) {
        Ok(date)  => Ok(build_cal(name, date.date()).to_string()),
        _ => Err(Failure(Status::raw(400))),
    };
}

fn build_cal(name: String, dob: Date<FixedOffset>) -> Calendar {
    let mut calendar = Calendar::new();

    let mut busy = Property::new("TRANSP", "TRANSPARENT");

    for x in 0..52 {
        calendar.push(Event::new()
            .all_day(dob.add(chrono::Duration::weeks(x)))
            .append_property(busy.done())
            .summary(&format!("{} is {} weeks old", name, x))
            .done());
    }
    return calendar
}

fn main() {
    rocket::ignite().mount("/",
        routes![index, lily, cal]
    ).launch();
}
