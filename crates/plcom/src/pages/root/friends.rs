use crate::prelude::*;
use rocket::http::uri::Absolute;

#[derive(Clone, PartialEq)]
struct Name {
    first: String,
    last: Option<String>,
}

impl Name {
    pub fn nick(nick: impl Into<String>) -> Self {
        Self {
            first: nick.into(),
            last: None,
        }
    }
    pub fn new(first: impl Into<String>, last: impl Into<String>) -> Self {
        Self {
            first: first.into(),
            last: Some(last.into()),
        }
    }

    pub fn initials(&self) -> String {
        let first = self
            .first
            .to_uppercase()
            .chars()
            .next()
            .expect("Invalid first name");
        let last = self
            .last
            .as_ref()
            .and_then(|last| last.to_uppercase().chars().next());

        match last {
            Some(last) => format!("{first}{last}"),
            None => first.into(),
        }
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.last {
            Some(last) => write!(f, "{} {}", self.first, last),
            None => write!(f, "{}", self.first),
        }
    }
}

#[derive(Clone, PartialEq)]
struct Friend {
    name: Name,
    uri: Absolute<'static>,
}

impl Friend {
    pub fn nick(nick: impl Into<String>, uri: Absolute<'static>) -> Self {
        Self {
            name: Name::nick(nick),
            uri,
        }
    }

    pub fn new(first: impl Into<String>, last: impl Into<String>, uri: Absolute<'static>) -> Self {
        Self {
            name: Name::new(first, last),
            uri,
        }
    }

    pub fn domain_name(&self) -> String {
        self.uri
            .authority()
            .map(|authority| authority.to_string())
            .unwrap_or_else(|| self.uri.to_string())
    }
}

impl IntoAny for Friend {
    fn into_any(self) -> AnyView {
        view! {
            <a
            href=self.uri.to_string()
            class=tw_join!(
                "hover:bg-gray-500", "transition-all", "duration-200", "flex", "items-center",
                "rounded-lg", "p-2"
            )
            >

            <span class=tw_join!(
                "rounded-full", "flex-shrink-0", "mr-4", "w-10", "h-10", "bg-sky-900", "text-white",
                "flex", "items-center", "justify-center", "text-lg", "font-medium"
            )>{self.name.initials()}</span>
            <div>
            <p class=tw_join!("font-bold")>{self.name.to_string()}</p>
                <p>{self.domain_name()}</p>
                </div>
                </a>
        }
        .into_any()
    }
}

pub fn friends() -> impl IntoView {
    let friends = [
        Friend::new("Paolo", "Rotolo", uri!("https://rotolo.dev")),
        Friend::new("Polly", "Bishop", uri!("https://github.com/itspolly")),
        Friend::new("Ayden", "Panhuyzen", uri!("https://ayden.dev")),
        Friend::new("Corbin", "Crutchley", uri!("https://crutchcorn.dev")),
        Friend::new("James", "Fenn", uri!("https://jfenn.me")),
        Friend::new("Alex", "Dueppen", uri!("https://ajd.sh")),
        Friend::new("Lyra", "Messier", uri!("https://lyramsr.co")),
        Friend::new("Peter", "Soboyejo", uri!("https://twitter.com/pxtvr")),
        Friend::nick("Millomaker", uri!("https://youtube.com/millomaker")),
        Friend::new("Alexandre", "Wagner", uri!("https://dev4people.fr")),
        Friend::new("Aidan", "Follestad", uri!("https://af.codes")),
        Friend::new("Victor", "Simon", uri!("https://simonvictor.com")),
        Friend::new("Guillaume", "Girol", uri!("https://github.com/symphorien")),
        Friend::new("Lara", "Kermarec", uri!("https://blog.nemirwen.me")),
    ];

    view! {
        <div>
            <h1 class=tw_join!("text-4xl", "font-bold", "mb-4")>"Friends"</h1>
            <p class=tw_join!("text-lg")>"Folks I worked with, or I like what they do."</p>

            <ul class=tw_join!(
                "my-4", "grid", "grid-cols-1", "sm:grid-cols-2", "md:grid-cols-3", "lg:grid-cols-4",
                "sm:gap-4"
            )>
                {friends
                    .into_iter()
                    .map(|friend| {
                        view! {
                            <li class=tw_join!("py-2")>
                            {friend.into_any()}
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>

            <p>"If you do not appear here and we know each other, hit me up!"</p>
        </div>
    }
}
