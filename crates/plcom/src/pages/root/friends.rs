use crate::prelude::*;

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
    uri: Uri,
}

impl Friend {
    pub fn nick(nick: impl Into<String>, uri: &'static str) -> Self {
        Self {
            name: Name::nick(nick),
            uri: Uri::from_static(uri),
        }
    }

    pub fn new(first: impl Into<String>, last: impl Into<String>, uri: &'static str) -> Self {
        Self {
            name: Name::new(first, last),
            uri: Uri::from_static(uri),
        }
    }

    pub fn domain_name(&self) -> String {
        self.uri
            .authority()
            .map(|authority| authority.to_string())
            .unwrap_or_else(|| self.uri.to_string())
    }
}

#[component]
fn Friend(#[prop(into)] friend: MaybeSignal<Friend>) -> impl IntoView {
    view! {
        <a
            href=friend.get().uri.to_string()
            target="_blank"
            class=tw_join!(
                "hover:bg-gray-500", "transition-all", "duration-200", "flex", "items-center",
                "rounded-lg", "p-2"
            )
        >

            <span class=tw_join!(
                "rounded-full", "flex-shrink-0", "mr-4", "w-10", "h-10", "bg-sky-900", "text-white",
                "flex", "items-center", "justify-center", "text-lg", "font-medium"
            )>{friend.get().name.initials()}</span>
            <div>
                <p class=tw_join!("font-bold")>{friend.get().name.to_string()}</p>
                <p>{friend.get().domain_name()}</p>
            </div>
        </a>
    }
}

#[component]
pub fn Friends() -> impl IntoView {
    let friends = [
        Friend::new("Paolo", "Rotolo", "https://rotolo.dev"),
        Friend::new("Polly", "Bishop", "https://github.com/itspolly"),
        Friend::new("Ayden", "Panhuyzen", "https://ayden.dev"),
        Friend::new("Corbin", "Crutchley", "https://crutchcorn.dev"),
        Friend::new("James", "Fenn", "https://jfenn.me"),
        Friend::new("Alex", "Dueppen", "https://ajd.sh"),
        Friend::new("Lyra", "Messier", "https://lyramsr.co"),
        Friend::new("Peter", "Soboyejo", "https://twitter.com/pxtvr"),
        Friend::nick("Millomaker", "https://youtube.com/millomaker"),
        Friend::new("Alexandre", "Wagner", "https://wagnerwave.com"),
        Friend::new("Aidan", "Follestad", "https://af.codes"),
        Friend::new("Victor", "Simon", "https://simonvictor.com"),
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
                    .map(|f| {
                        view! {
                            <li class=tw_join!("py-2")>
                                <Friend friend=f/>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>

            <p>"If you do not appear here and we know each other, hit me up!"</p>
        </div>
    }
}
