#[macro_use] extern crate rocket;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::serde::{Serialize, json};

pub struct CORS;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Ticket {
    title: String,
    description: String
}

type Tickets = Vec<Ticket>;

impl Ticket {
    fn from_text(text: impl AsRef<str>) -> Self {
        let words: Vec<&str> = text.as_ref().split(" ").collect();
        Ticket{title: words[0].to_owned(), description: text.as_ref().to_owned()}
    }
}

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/tickets")]
fn tickets() -> json::Json<Tickets> {
    let tickets: Vec::<Ticket> = [
        "Work on this task",
        "And then on this",
        "Finally work on this",
    ].into_iter().map(|t| Ticket::from_text(t)).collect();
    json::Json(tickets)
    
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, tickets])
        .attach(CORS)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ticket_from_text() {
        let t = Ticket::from_text("Work on this task");
        assert_eq!(t.title, "Work");
        assert_eq!(t.description, "Work on this task");
    }
}