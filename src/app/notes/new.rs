use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    router::{
        content::Form,
        error::{SeeOther, see_other},
        href, page, route,
    },
    view::{View, component, view},
};

use crate::data;

#[page]
async fn page() -> Result<impl View> {
    Ok(view! {
        <a class="font-mono text-xs text-primary" href="/notes">"← NOTEBOOK"</a>
        <div class="mx-auto mt-8 max-w-3xl">
            <p class="font-mono text-xs uppercase tracking-[.2em] text-primary">"NEW FIELDNOTE"</p>
            <h1 class="mt-3 text-4xl">"Put the experiment on record."</h1>
            <p class="mt-3 max-w-2xl opacity-65">"Capture the question, the attempt, and what you learned. Format it like a document, not a textarea."</p>
            <form method="post" action="/notes/new" class="mt-8 grid gap-5">
                <label class="grid gap-2 text-sm">"Title"
                    <input class="border border-black/20 bg-white/40 px-3 py-3" name="title" maxlength="120" placeholder="A question worth following" required="">
                </label>
                <label class="grid gap-2 text-sm">"Topic"
                    <input class="max-w-sm border border-black/20 bg-white/40 px-3 py-3" name="topic" maxlength="48" placeholder="systems, design, rust..." required="">
                </label>
                <div class="grid gap-2 text-sm">
                    <label for="note-body">"Fieldnote"</label>
                    <span class="text-xs opacity-60">"Select text to format it: headings, bold, italic, lists, quote, and code."</span>
                    editor_toolbar()
                    <textarea
                        class="min-h-64 w-full border border-black/20 bg-white/50 p-4 font-serif leading-7"
                        id="note-body"
                        data-editor-source=""
                        name="body"
                        placeholder="What did you try? What surprised you? What will you try next?"
                        required=""
                    ></textarea>
                    <div class="hidden min-h-64 border border-black/20 bg-white/50 p-4 font-serif leading-7" data-editor-surface=""></div>
                </div>
                <div class="flex justify-end gap-3">
                    <a class="border border-black/20 px-4 py-3 text-sm" href="/notes">"Cancel"</a>
                    <button class="bg-primary px-5 py-3 font-mono text-sm text-white" type="submit">"Publish fieldnote"</button>
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
async fn create(cx: &Cx, Form(form): Form<NoteForm>) -> Result<SeeOther> {
    let title = form.title.trim();
    let topic = form.topic.trim();
    let body = data::sanitize_rich_text(form.body.trim());
    if title.is_empty()
        || title.chars().count() > 120
        || topic.is_empty()
        || topic.chars().count() > 48
        || !data::rich_text_has_content(&body)
    {
        return Ok(see_other(href!(page).resolve(cx)));
    }

    let note = data::create(cx, title.to_owned(), topic.to_owned(), body).await?;
    Ok(see_other(
        href!(super::id::page, super::id::Id(note.public_id)).resolve(cx),
    ))
}
