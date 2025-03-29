use maud::{DOCTYPE, Markup, html};

use crate::models::calypso_item::CalypsoItem;

pub fn layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) " - Millions" }
                link rel="stylesheet" href="/styles.css";
            }
            body {
                nav.bottom-nav {
                    ul {
                        li { a href="/news" { "📰" br; "News" } }
                    }
                }
                main {
                    (content)
                }
            }
        }
    }
}

pub fn not_found() -> Markup {
    let content = html! {
        h1 { "404 Not Found" }
    };
    layout("404 Not Found", content)
}

pub fn news_feed(items: &Vec<CalypsoItem>) -> Markup {
    html! {
        @for item in items {
            .card {
                h2 { (item.title_english) }
                p {
                    "By " (item.author_display)
                    @if let Some(p_as) = item.publish_as.as_ref() { " as " (p_as) }
                    " · " (item.publish_date.datetime())
                }
                @if item.item_type == "EVENT" {
                    p {
                        "📅 " (item.event_start_time.as_ref().unwrap().date())
                        " 🕒 " (item.event_start_time.as_ref().unwrap().time()) " - " (item.event_end_time.as_ref().unwrap().time())
                        " 📍 " (item.event_location.as_ref().unwrap())
                    }
                }
            }
        }
    }
}
