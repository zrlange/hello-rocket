#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hola, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

//addting a comment for git
