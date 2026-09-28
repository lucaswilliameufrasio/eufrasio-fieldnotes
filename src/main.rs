mod app;
mod data;
mod migrations;
mod models;

use toasty::Db;

use crate::models::Note;

#[tokio::main]
async fn main() {
    let initialize_schema = !std::path::Path::new("fieldnotes.sqlite").exists();
    if !initialize_schema {
        migrations::ensure_public_ids("fieldnotes.sqlite").unwrap();
    }
    let db = Db::builder()
        .models(toasty::models!(crate::*))
        .connect("sqlite:fieldnotes.sqlite")
        .await
        .unwrap();
    if initialize_schema {
        db.push_schema().await.unwrap();
        migrations::ensure_public_ids("fieldnotes.sqlite").unwrap();
    }

    if Note::all().exec(&mut db.clone()).await.unwrap().is_empty() {
        let body = ammonia::clean(
            "<h2>The question</h2><p>Can a small web framework make server-side work feel immediate without moving the whole application into the browser?</p><h2>What I tried</h2><p>I split the experience into an index, a full reading view, and a focused editor. The search stays in the browser until it needs fresh results; the note itself remains a server-rendered record.</p><ul><li><strong>signal</strong> keeps typing local.</li><li><strong>shard</strong> asks the server for matching notes.</li><li><strong>Toasty</strong> keeps the published record durable.</li></ul><blockquote><p>Keep the page small enough to understand; keep the product large enough to use.</p></blockquote>",
        );
        toasty::create!(Note {
            public_id: uuid::Uuid::now_v7().to_string(),
            title: "Can server-rendered apps still feel immediate?".to_owned(),
            topic: "topcoat / field test".to_owned(),
            body,
        })
        .exec(&mut db.clone())
        .await
        .unwrap();
    }

    topcoat::start(app::router(db)).await.unwrap();
}
