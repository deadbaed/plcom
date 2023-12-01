use rocket::http::uri::Absolute;
use rocket::uri;
pub enum Icon {
    Email,
    Github,
    Linkedin,
    Mastodon,
    Telegram,
    Twitter,
}

pub struct Network {
    pub name: &'static str,
    pub uri: Absolute<'static>,
    pub icon: Icon,
}
impl Network {
    pub fn new() -> Vec<Self> {
        vec![
            Network {
                name: "Twitter",
                uri: uri!("https://twitter.com/philippeloctaux"),
                icon: Icon::Twitter,
            },
            Network {
                name: "Telegram",
                uri: uri!("https://t.me/philippeloctaux"),
                icon: Icon::Telegram,
            },
            Network {
                name: "Mastodon",
                uri: uri!("https://mastodon.social/@philt3r"),
                icon: Icon::Mastodon,
            },
            Network {
                name: "GitHub",
                uri: uri!("https://github.com/x4m3"),
                icon: Icon::Github,
            },
            Network {
                name: "LinkedIn",
                uri: uri!("https://linkedin.com/in/philippeloctaux"),
                icon: Icon::Linkedin,
            },
            Network {
                name: "Email",
                uri: uri!("https://philippeloctaux.com/email"),
                icon: Icon::Email,
            },
        ]
    }
}
