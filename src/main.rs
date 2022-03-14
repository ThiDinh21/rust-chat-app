use rocket::{
    form::Form,
    routes,
    serde::{Deserialize, Serialize},
    tokio::sync::broadcast::{channel, Sender},
    FromForm, State,
};

#[macro_use]
extern crate rocket;

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    let _res = queue.send(form.into_inner());
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/hello", routes![world, post])
}
