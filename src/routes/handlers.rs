use crate::{
    AppState,
    db::posts::store_post,
    errors::http::ServerError,
    extractors::Json,
    models::{
        expo_token::ExpoToken,
        post::{Event, Post, Translation},
    },
    services::calypso::fetch_events,
    templates::{layout, news_feed},
};
use axum::{Form, extract::State};
use maud::{Markup, html};
use reqwest::Client;
use serde::Deserialize;

pub async fn store_token(
    State(state): State<AppState>,
    Json(token): Json<ExpoToken>,
) -> Result<(), ServerError> {
    crate::db::tokens::store_token(state.db.as_ref(), &token.token)?;
    Ok(())
}

pub async fn news(State(state): State<AppState>) -> Result<Markup, ServerError> {
    let items = fetch_events(&state.client).await?;
    let content = html! {
        section.news {
            h2 { "Millions News Feed" }
            (news_feed(&items))
        }
    };
    Ok(layout("Home", content))
}

// The page where the user can create a post
pub async fn get_create_post() -> Result<Markup, ServerError> {
    let content = html! {
        section.create-post {
            form method="POST" action="/posts/create" {
                h1 { "Create New Post" }

                // Main post fields
                .form-group {
                    label for="title" { "Post Title" }
                    input type="text" id="title" name="title" required;
                }

                .form-group {
                    label for="publish" { "Publish Date/Time" }
                    input type="datetime-local" id="publish" name="publish";
                }

                // Translations section
                fieldset {
                    legend { "Translations" }

                    .translation {
                        .form-group {
                            label for="translation_language" { "Language Code (e.g., 'en')" }
                            input type="text" id="translation_language" name="translations[0][language]"
                                pattern="[a-z]{2}" title="2-letter language code" required;
                        }

                        .form-group {
                            label for="translation_title" { "Translated Title" }
                            input type="text" id="translation_title" name="translations[0][title]" required;
                        }

                        .form-group {
                            label for="translation_description" { "Translated Description" }
                            textarea id="translation_description" name="translations[0][description]" required { }
                        }
                    }

                    button type="button" class="add-translation" {
                        "Add Another Translation"
                    }
                }

                // Event fields (optional)
                fieldset {
                    legend { "Event Details (Optional)" }
                    div.form-group {
                        label for="location" { "Location" }
                        input type="text" id="location" name="event[location]";
                    }
                    div.form-group {
                        label for="start_time" { "Start Time" }
                        input type="datetime-local" id="start_time" name="event[start_time]";
                    }
                    div.form-group {
                        label for="end_time" { "End Time" }
                        input type="datetime-local" id="end_time" name="event[end_time]";
                    }
                }

                button type="submit" { "Create Post" }
            }
        }
    };
    Ok(layout("Create Post", content))
}

pub async fn post_create_post(
    State(state): State<AppState>,
    Form(form): Form<Post>,
) -> Result<(), ServerError> {
    let conn = state.postgres.get_owned().await?;
    tracing::info!(?form, "event form received");
    store_post(conn, form).await?;
    Ok(())
}
