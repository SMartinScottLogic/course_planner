use rocket::data::{Limits, ToByteUnit};
use rocket::serde::json::Json;
use rocket::State;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate rocket;

use common::{Course, Stage};

#[derive(Default, Debug)]
struct Config {
    courses: Arc<Mutex<HashMap<String, Course>>>,
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/courses")]
fn courses(state: &State<Config>) -> Json<Vec<(String, String)>> {
    let r = state
        .courses
        .lock()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.to_owned(), v.name().to_owned()))
        .collect();
    Json(r)
}

#[get("/course/<id>")]
fn course(state: &State<Config>, id: &str) -> Option<Json<Vec<Stage>>> {
    state
        .courses
        .lock()
        .unwrap()
        .get(id)
        .map(Course::stages)
        .map(|i| i.collect())
        .map(Json)
}

#[post("/course/<id>", data = "<stage>")]
fn add_stage(state: &State<Config>, id: &str, stage: Json<Stage>) -> Json<Vec<Stage>> {
    let mut courses = state.courses.lock().unwrap();
    let course = courses
        .entry(id.to_string())
        .or_insert_with(|| Course::new(""));
    course.add(stage.into_inner());
    Json(course.stages().collect())
}

#[put("/course/<id>")]
fn add_course(state: &State<Config>, id: &str) -> Json<Vec<Stage>> {
    let mut courses = state.courses.lock().unwrap();
    let course = courses
        .entry(id.to_string())
        .or_insert_with(|| Course::new(""));
    Json(course.stages().collect())
}

/*
#[get("/test/<id>")]
fn test(id: &str) -> Option<Json<Vec<Stage>>> {
    COURSES
        .lock()
        .as_mut()
        .map(|courses| {
            let course = courses.entry(id.to_string()).or_insert_with(|| {
                let mut course = Course::default();
                for (name, duration) in &[
                    ("Gravy", "2min"),
                    ("Roast Potatoes", "30min"),
                    ("Yorkshire puddings", "25min"),
                ] {
                    course.add(Stage::new(name, duration));
                }
                for stage in Stage::chain(vec![
                    Stage::new("Duck crown", "1h 15min"),
                    Stage::new("Duck Legs", "15min"),
                    Stage::new("Duck Legs with sauce", "15min"),
                    Stage::new("Reduce sauce", "2min"),
                ]) {
                    course.add(stage);
                }
                course
            });
            course
        })
        .map(|course| course.stages())
        .map(Iterator::collect)
        .map(Json)
        .ok()
}
*/

#[launch]
fn rocket() -> _ {
    let config = Config::default();

    let figment = rocket::Config::figment()
        .merge(("port", 1111))
        .merge(("limits", Limits::new().limit("json", 2.mebibytes())))
        .merge(("tls.certs", "backend/certs.pem"))
        .merge(("tls.key", "backend/key.pem"));
    rocket::custom(figment)
        .mount(
            "/",
            routes![
                hello, //test,
                courses, course, add_stage, add_course
            ],
        )
        .manage(config)
}
