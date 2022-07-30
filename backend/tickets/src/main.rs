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
        "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo", 
        "ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis",
        "dis parturient montes, nascetur ridiculus mus. Donec quam felis,",
        "ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa",
        "quis enim. Donec pede justo, fringilla vel, aliquet nec, vulputate eget,",
        "arcu. In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo.",
        "Nullam dictum felis eu pede mollis pretium. Integer tincidunt. Cras",
        "dapibus. Vivamus elementum semper nisi. Aenean vulputate eleifend",
        "tellus. Aenean leo ligula, porttitor eu, consequat vitae, eleifend ac,",
        "enim. Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus.",
        "Phasellus viverra nulla ut metus varius laoreet. Quisque rutrum. Aenean",
        "imperdiet. Etiam ultricies nisi vel augue. Curabitur ullamcorper",
        "ultricies nisi. Nam eget dui. Etiam rhoncus. Maecenas tempus, tellus",
        "eget condimentum rhoncus, sem quam semper libero, sit amet adipiscing",
        "sem neque sed ipsum. Nam quam nunc, blandit vel, luctus pulvinar,",
        "hendrerit id, lorem. Maecenas nec odio et ante tincidunt tempus. Donec",
        "vitae sapien ut libero venenatis faucibus. Nullam quis ante. Etiam sit",
        "amet orci eget eros faucibus tincidunt. Duis leo. Sed fringilla mauris",
        "sit amet nibh. Donec sodales sagittis magna. Sed consequat, leo eget",
        "bibendum sodales, augue velit cursus nunc,",
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