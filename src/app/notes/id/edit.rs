use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{
        content::Form,
        error::{RouterErrorExt, SeeOther, see_other},
        href, page, path_param, route,
    },
    view::{View, component, view},
};

use super::Id;
use crate::data;

#[page]
async fn page(cx: &Cx) -> Result<impl View> {
    let public_id: &str = path_param::<Id>(cx);
    let note = data::find(cx, public_id).await?.ok_or_not_found()?;
    let safe_body = data::sanitize_rich_text(&note.body);

    Ok(view! {
        <a class="font-mono text-xs text-primary" href=(format!("/notes/{public_id}"))>"← READ FIELDNOTE"</a>
        <div class="mx-auto mt-8 max-w-3xl">
            <p class="font-mono text-xs uppercase tracking-[.2em] text-primary">"EDIT FIELDNOTE / " (note.public_id)</p>
            <h1 class="mt-3 text-4xl">"Revise the record."</h1>
            <form method="post" action=(format!("/notes/{public_id}/edit")) class="mt-8 grid gap-5">
                <label class="grid gap-2 text-sm">"Title"
                    <input class="border border-black/20 bg-white/40 px-3 py-3" name="title" value=(note.title) maxlength="120" required="">
                </label>
                <label class="grid gap-2 text-sm">"Topic"
                    <input class="max-w-sm border border-black/20 bg-white/40 px-3 py-3" name="topic" value=(note.topic) maxlength="48" required="">
                </label>
                <div class="grid gap-2 text-sm">
                    <label for="note-body">"Fieldnote"</label>
                    <span class="text-xs opacity-60">"The saved rich text is loaded into the editor."</span>
                    editor_toolbar()
                    <textarea class="min-h-64 w-full border border-black/20 bg-white/50 p-4 font-serif leading-7" id="note-body" data-editor-source="" name="body" required="">(safe_body)</textarea>
                    <div class="hidden min-h-64 border border-black/20 bg-white/50 p-4 font-serif leading-7" data-editor-surface=""></div>
                </div>
                <div class="flex justify-end gap-3">
                    <a class="border border-black/20 px-4 py-3 text-sm" href=(format!("/notes/{public_id}"))>"Cancel"</a>
                    <button class="bg-primary px-5 py-3 font-mono text-sm text-white" type="submit">"Save changes"</button>
                </div>
            </form>
        </div>
    })
}

#[component]
async fn editor_toolbar() -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-wrap gap-1 border border-black/20 bg-black/[.03] p-2" data-editor-toolbar="">
            <button class="editor-tool" type="button" data-command="bold"><strong>"B"</strong></button>
            <button class="editor-tool italic" type="button" data-command="italic">"I"</button>
            <button class="editor-tool" type="button" data-command="heading1">"H1"</button>
            <button class="editor-tool" type="button" data-command="heading2">"H2"</button>
            <button class="editor-tool" type="button" data-command="bulletList">"• List"</button>
            <button class="editor-tool" type="button" data-command="blockquote">"Quote"</button>
            <button class="editor-tool" type="button" data-command="codeBlock">"Code"</button>
            <button class="editor-tool" type="button" data-command="undo">"Undo"</button>
            <button class="editor-tool" type="button" data-command="redo">"Redo"</button>
        </div>
    })
}

#[derive(Deserialize)]
struct NoteForm {
    title: String,
    topic: String,
    body: String,
}

#[route(POST)]
async fn update(cx: &Cx, Form(form): Form<NoteForm>) -> Result<SeeOther> {
    let public_id: &str = path_param::<Id>(cx);
    let title = form.title.trim();
    let topic = form.topic.trim();
    let body = data::sanitize_rich_text(form.body.trim());
    if !title.is_empty()
        && title.chars().count() <= 120
        && !topic.is_empty()
        && topic.chars().count() <= 48
        && data::rich_text_has_content(&body)
    {
        data::update(cx, public_id, title.to_owned(), topic.to_owned(), body).await?;
    }
    Ok(see_other(
        href!(super::page, Id(public_id.to_owned())).resolve(cx),
    ))
}
