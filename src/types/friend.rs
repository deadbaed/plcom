use rocket::http::uri::Absolute;
use rocket::uri;

pub struct Friend {
    pub first_name: &'static str,
    pub last_name: &'static str,
    pub uri: Absolute<'static>,
}

impl Friend {
    pub fn initials(&self) -> String {
        let first = self
            .first_name
            .to_uppercase()
            .chars()
            .next()
            .expect("Invalid first name");
        let last = self
            .last_name
            .to_uppercase()
            .chars()
            .next()
            .expect("Invalid last name");

        format!("{first}{last}")
    }

    pub fn domain_name(&self) -> String {
        match self.uri.authority() {
            Some(authority) => authority.to_string(),
            None => self.uri.to_string(),
        }
    }

    pub fn new() -> Vec<Self> {
        vec![
            Friend {
                first_name: "Jamie",
                last_name: "Bishop",
                uri: uri!("https://jamiebi.shop"),
            },
            Friend {
                first_name: "Ayden",
                last_name: "Panhuyzen",
                uri: uri!("https://ayden.dev"),
            },
            Friend {
                first_name: "Corbin",
                last_name: "Crutchley",
                uri: uri!("https://crutchcorn.dev"),
            },
            Friend {
                first_name: "James",
                last_name: "Fenn",
                uri: uri!("https://jfenn.me"),
            },
            Friend {
                first_name: "Alex",
                last_name: "Dueppen",
                uri: uri!("https://ajd.sh"),
            },
            Friend {
                first_name: "Peter",
                last_name: "Sobolev",
                uri: uri!("https://petersoboyejo.com"),
            },
            Friend {
                first_name: "Alexandre",
                last_name: "Wagner",
                uri: uri!("https://wagnerwave.com"),
            },
            Friend {
                first_name: "Aidan",
                last_name: "Follestad",
                uri: uri!("https://af.codes"),
            },
            Friend {
                first_name: "Victor",
                last_name: "Simon",
                uri: uri!("https://simonvictor.com"),
            },
        ]
    }
}
