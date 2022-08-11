use rocket::{
  serde::json::{Json, Value},
  State,
};

mod cors;
mod logs;

#[rocket::post("/answers", format = "json", data = "<data>")]
fn answers(data: Json<Value>, logs: &State<logs::LogFiles>) -> &'static str {
  logs.append("answers.log", &data.to_string()).unwrap();
  "success"
}

#[rocket::post("/bug", format = "json", data = "<data>")]
fn bug(data: Json<Value>, logs: &State<logs::LogFiles>) -> &'static str {
  logs.append("bug.log", &data.to_string()).unwrap();
  "success"
}

#[rocket::post("/feedback", format = "json", data = "<data>")]
fn feedback(data: Json<Value>, logs: &State<logs::LogFiles>) -> &'static str {
  logs.append("feedback.log", &data.to_string()).unwrap();
  "success"
}

#[rocket::get("/")]
fn index() -> &'static str {
  "MIND OVER COMPUTER"
}

#[rocket::launch]
fn rocket() -> _ {
  rocket::build()
    .attach(cors::CORS)
    .manage(logs::LogFiles::new(vec!["answers.log", "bug.log", "feedback.log"]))
    .mount("/", rocket::routes![index, answers, bug, feedback, cors::all_options])
}
