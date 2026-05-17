use crate::prelude::*;

pub struct Blog<'a> {
    path: &'a str,
    feed: Option<atom_syndication::Feed>,
}

#[derive(Debug, thiserror::Error)]
pub enum BlogError {
    #[error("Failed to read file: {0}")]
    ReadFile(rocket::tokio::io::ErrorKind),

    #[error("Failed to parse Atom feed: {0}")]
    ParseFeed(atom_syndication::Error),
}

impl<'a> Blog<'a> {
    pub fn new(url: &'a str) -> Self {
        Self {
            path: url,
            feed: None,
        }
    }

    pub async fn fetch_feed(&mut self) -> Result<(), BlogError> {
        let mut file = rocket::tokio::fs::File::open(self.path)
            .await
            .map_err(|e| BlogError::ReadFile(e.kind()))?;

        use rocket::tokio::io::AsyncReadExt;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .await
            .map_err(|e| BlogError::ReadFile(e.kind()))?;

        let blog_feed = buffer
            .parse::<atom_syndication::Feed>()
            .map_err(BlogError::ParseFeed)?;

        self.feed = Some(blog_feed);
        Ok(())
    }

    fn view_entry(entry: atom_syndication::Entry) -> impl IntoAny {
        let timestamp = entry.updated.timestamp();
        let date = jiff::Timestamp::from_second(timestamp)
            .unwrap()
            .strftime("%B %d, %Y")
            .to_string();

        let link = entry
            .links()
            .first()
            .map(|link| {
                let link = Link::parse(&link.href, "Read post");
                view! { {outline_button_link(link).into_any()} }
            })
            .unwrap_or(().into_any());

        view! {
            <div class=tw_join!("rounded-2xl", "w-full", "bg-purple-950", "p-6")>

                <div class=tw_join!(
                    "text-xl", "font-semibold", "mb-2"
                )>{entry.title.to_string()}</div>
                <div class=tw_join!("flex")>
                    <div class=tw_join!(
                        "inline-flex", "items-center"
                    )>{Icon::Calendar.into_any()} <span class=tw_join!("ml-2")>{date}</span></div>
                </div>

                {link.into_any()}
            </div>
        }
    }
}

impl<'a> IntoAny for Blog<'a> {
    fn into_any(self) -> AnyView {
        let title = match self.feed {
            Some(_) => "Latest blog posts",
            None => "Blog",
        };

        let paragraph = if self.feed.is_none() {
            view! {
                <div class=tw_join!("text-lg", "space-y-6")>
                    <p>
                        "I like to share my knowledge on a specific topic that can be beneficial for others."
                    </p>
                    <p>"It helps me become a better writter."</p>
                </div>
            }.into_any()
        } else {
            ().into_any()
        };

        let entries = self
            .feed
            .map(|feed| {
                feed.entries
                    .into_iter()
                    .take(3)
                    .map(Self::view_entry)
                    .map(IntoAny::into_any)
                    .collect_view()
            })
            .unwrap_or_default();

        let link = Link::new(
            uri!("https://philippeloctaux.com/blog").into(),
            "See all posts",
        );

        view! {
            <div>
                <h1 class=tw_join!("text-4xl", "font-bold", "mb-4")>{title}</h1>

                <div class=tw_join!(
                    "grid", "grid-cols-1", "sm:grid-cols-2", "lg:grid-cols-3", "gap-6",
                "place-content-center"
                )>{entries}</div>

                {paragraph}

                <div class=tw_join!(
                    "mt-4"
                )>{button_link(link, Some(Icon::Link), None).into_any()}</div>
            </div>
        }
        .into_any()
    }
}
