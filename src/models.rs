#[derive(Debug, toasty::Model)]
pub(crate) struct Note {
    #[key]
    #[auto]
    pub(crate) id: i64,
    pub(crate) public_id: String,
    pub(crate) title: String,
    pub(crate) topic: String,
    /// Sanitized HTML emitted by the rich-text editor.
    pub(crate) body: String,
}

#[cfg(test)]
mod tests {
    use super::Note;
    use toasty::Db;

    #[tokio::test]
    async fn rich_note_can_be_created_read_and_updated_in_sqlite() {
        let mut db = Db::builder()
            .models(toasty::models!(crate::*))
            .connect("sqlite::memory:")
            .await
            .unwrap();
        db.push_schema().await.unwrap();

        let mut note = toasty::create!(Note {
            public_id: uuid::Uuid::now_v7().to_string(),
            title: "First title".to_owned(),
            topic: "rust".to_owned(),
            body: "<p><strong>formatted</strong> body</p>".to_owned(),
        })
        .exec(&mut db)
        .await
        .unwrap();
        toasty::update!(note {
            title: "Revised title".to_owned(),
        })
        .exec(&mut db)
        .await
        .unwrap();

        let saved = Note::filter_by_id(note.id)
            .first()
            .exec(&mut db)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(saved.title, "Revised title");
        assert_eq!(saved.body, "<p><strong>formatted</strong> body</p>");
        assert_eq!(saved.public_id.as_bytes()[14], b'7');
        let found = Note::filter(Note::fields().public_id().eq(note.public_id.clone()))
            .first()
            .exec(&mut db)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(found.id, note.id);
    }
}
