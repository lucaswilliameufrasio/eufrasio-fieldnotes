use toasty::Db;
use topcoat::{
    Result,
    context::{Cx, app_context},
};

use crate::models::Note;

pub(crate) fn sanitize_rich_text(html: &str) -> String {
    ammonia::clean(html)
}

pub(crate) fn rich_text_has_content(html: &str) -> bool {
    let mut in_tag = false;
    let mut text = String::new();
    for character in html.chars() {
        match character {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => text.push(character),
            _ => {}
        }
    }
    !text
        .replace("&nbsp;", " ")
        .replace("&#160;", " ")
        .trim()
        .is_empty()
}

fn db(cx: &Cx) -> Db {
    app_context::<Db>(cx).clone()
}

pub(crate) async fn list(cx: &Cx) -> Result<Vec<Note>> {
    Note::all().exec(&mut db(cx)).await.map_err(Into::into)
}

pub(crate) async fn find(cx: &Cx, public_id: &str) -> Result<Option<Note>> {
    Note::filter(Note::fields().public_id().eq(public_id.to_owned()))
        .first()
        .exec(&mut db(cx))
        .await
        .map_err(Into::into)
}

pub(crate) async fn create(cx: &Cx, title: String, topic: String, body: String) -> Result<Note> {
    toasty::create!(Note {
        public_id: uuid::Uuid::now_v7().to_string(),
        title,
        topic,
        body,
    })
    .exec(&mut db(cx))
    .await
    .map_err(Into::into)
}

pub(crate) async fn update(
    cx: &Cx,
    public_id: &str,
    title: String,
    topic: String,
    body: String,
) -> Result<bool> {
    let Some(mut note) = find(cx, public_id).await? else {
        return Ok(false);
    };

    toasty::update!(note { title, topic, body })
        .exec(&mut db(cx))
        .await?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::{rich_text_has_content, sanitize_rich_text};

    #[test]
    fn rich_text_validation_rejects_empty_editor_markup() {
        assert!(!rich_text_has_content("<p></p>"));
        assert!(!rich_text_has_content("<p>&nbsp;</p>"));
        assert!(rich_text_has_content("<p>One useful note.</p>"));
    }

    #[test]
    fn rich_text_sanitization_removes_script_markup() {
        let html = sanitize_rich_text("<p>safe</p><script>alert(1)</script>");
        assert!(html.contains("<p>safe</p>"));
        assert!(!html.contains("<script"));
    }
}
