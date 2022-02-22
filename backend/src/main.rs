use rocket::data::{Limits, ToByteUnit};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{Request, Response};

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate rocket;

use common::{Course, CourseDetails, Stage};

#[derive(Default, Debug)]
struct Config {
    courses: Arc<Mutex<HashMap<String, Course>>>,
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/courses")]
fn get_courses(state: &State<Config>) -> Json<Vec<CourseDetails>> {
    let r = state
        .courses
        .lock()
        .unwrap()
        .iter()
        .map(|(_k, v)| (v.details().to_owned()))
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
    let stages = match courses.get_mut(id) {
        Some(course) => {
            course.add(stage.into_inner());
            course.stages().collect()
        }
        None => Vec::new(),
    };
    Json(stages)
}

#[put("/course", data = "<details>")]
fn add_course(state: &State<Config>, details: Json<CourseDetails>) -> Json<Vec<CourseDetails>> {
    {
        let mut courses = state.courses.lock().unwrap();
        let mut details = details.into_inner();
        let id = uuid::Uuid::new_v4().to_string();
        details.set_id(&id);
        let mut course = Course::new(&details);
        for (i, s) in details.name().split(' ').enumerate() {
            let stage = Stage::new(s, &format!("{}s", 30 + i * 20));
            course.add(stage);
        }
        courses.insert(id, course);
    }
    get_courses(state)
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

#[options("/<_path..>")]
fn options(_path: PathBuf) -> String {
    "Ok".to_string()
}

struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS, PUT",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

struct NoCache;

#[rocket::async_trait]
impl Fairing for NoCache {
    fn info(&self) -> Info {
        Info {
            name: "Attaching NoCache headers to responses",
            kind: Kind::Response,
        }
    }
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
            "Cache-Control",
            "no-cache, no-store, must-revalidate",
        ));
        response.set_header(Header::new("Pragma", "no-cache"));
        response.set_header(Header::new("Expires", "0"));
    }
}

#[launch]
fn rocket() -> _ {
    let config = Config::default();

    let figment = rocket::Config::figment()
        .merge(("port", 1111))
        .merge(("limits", Limits::new().limit("json", 2.mebibytes())))
        .merge(("tls.certs", "backend/certs.pem"))
        .merge(("tls.key", "backend/key.pem"));
    rocket::custom(figment)
        .attach(Cors)
        .attach(NoCache)
        .mount(
            "/",
            routes![
                hello, //test,
                get_courses,
                course,
                add_stage,
                add_course,
                options,
            ],
        )
        .manage(config)
}
