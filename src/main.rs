#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[get("/about")]
fn about() -> &'static str {
  "This is the about page of my Rocket Demo app"
}

#[launch]
fn rocket() -> _ {
  //rocket::build().mount("/", routes![index])
  rocket::build().mount("/about", routes![index])
}

