pub(crate) mod edit;

use topcoat::{
    Result,
    context::Cx,
    router::{error::RouterErrorExt, page, path_param},
    view::{Unescaped, View, view},
};

use crate::data;

path_param!(pub id);

#[page]
pub(crate) async fn page(cx: &Cx) -> Result<impl View> {
    let public_id: &str = path_param::<Id>(cx);
    let note = data::find(cx, public_id).await?.ok_or_not_found()?;
    let safe_body = data::sanitize_rich_text(&note.body);

    Ok(view! {
        <div class="flex items-center justify-between gap-4">
            <a class="font-mono text-xs text-primary" href="/notes">"← ALL FIELDNOTES"</a>
            <a class="border border-black/20 px-4 py-2 font-mono text-xs hover:border-primary" href=(format!("/notes/{public_id}/edit"))>
                "EDIT NOTE"
            </a>
        </div>
        <article class="mx-auto mt-10 max-w-3xl">
            <header class="border-b border-black/15 pb-7">
                <p class="font-mono text-xs uppercase tracking-[.2em] text-primary">(&note.topic) " / FIELDNOTE ID"</p>
                <h1 class="mt-4 text-4xl leading-tight tracking-[-.04em] md:text-6xl">(&note.title)</h1>
                <p class="mt-5 break-all font-mono text-xs opacity-45">(note.public_id)</p>
            </header>
            <div class="fieldnote-content mt-9 font-serif text-lg leading-8">
                (Unescaped::new_unchecked(safe_body))
            </div>
            <footer class="mt-12 border-t border-black/15 pt-6">
                <a class="font-mono text-xs text-primary" href="/notes">"← BACK TO THE NOTEBOOK"</a>
            </footer>
        </article>
    })
}
