 
#[macro_use] extern crate rocket;
 
use rocket::fs::FileServer;
use colored::Colorize;  

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let version = env!("CARGO_PKG_VERSION");


    println!("{}", format!("Caris Progress Tracker APP").green().underline());
    println!("{} {version:?}",format!("Version:").yellow().italic());

    let rocket = rocket::build();

     
    rocket
    .mount("/static", FileServer::from("./frontend") )
    .mount("/", routes![index])
}