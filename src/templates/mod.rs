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

fn translation(legend: &str, key: &str) -> Markup {
    html! {
        fieldset {
            legend { (legend) }

            .translation {
                input type="hidden" id="translation_language" name={ "translations[" (key) "][language]" } value=(key);

                .form-group {
                    label for="translation_title" { "Translated Title" }
                    input type="text" id="translation_title" name={ "translations[" (key) "][title]" } required;
                }

                .form-group {
                    label for="translation_description" { "Translated Description" }
                    textarea id="translation_description" name={ "translations[" (key) "][description]" } required { }
                }
            }
        }
    }
}

fn translations() -> Markup {
    html! {
        fieldset {
            legend { "Translations" }

            (translation("English 🇬🇧", "en"))

            (translation("Swedish 🇸🇪", "sv"))
        }
    }
}

fn post_form_base(name: &str, action: &str, extra: Option<Markup>) -> Markup {
    html! {
        section.create-form {
            form method="POST" action=(action) {
                h1 { "Create New " (name) }

                // Main form fields
                .form-group {
                    label for="title" { (name) " Title" }
                    input type="text" id="title" name="title" required;
                }

                .form-group {
                    label for="publish" { "Publish Date/Time" }
                    input type="datetime-local" id="publish" name="publish";
                }

                // Translations section
                (translations())


                // Maybe event fields
                @if let Some(e) = extra {
                    (e)
                }

                button type="submit" { "Create " (name) }
            }
        }
    }
}

fn event_extra() -> Markup {
    html! {
        fieldset {
            legend { "Event Details" }
            div.form-group {
                label for="location" { "Location" }
                input type="text" id="location" name="event[location]" required;
            }
            div.form-group {
                label for="start_time" { "Start Time" }
                input type="datetime-local" id="start_time" name="event[start_time]" required;
            }
            div.form-group {
                label for="end_time" { "End Time" }
                input type="datetime-local" id="end_time" name="event[end_time]" required;
            }
        }
    }
}

pub fn post_form() -> Markup {
    post_form_base("Post", "/posts/create", None)
}

pub fn event_form() -> Markup {
    post_form_base("Event", "/events/create", Some(event_extra()))
}
