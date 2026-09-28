pub(crate) mod id;
pub(crate) mod new;

use topcoat::{
    Result,
    context::Cx,
    router::page,
    runtime::{Event, shard, signal},
    view::{View, view},
};

use crate::data;

#[page]
async fn page(cx: &Cx) -> Result<impl View> {
    let query = signal(cx, String::new);

    Ok(view! {
        <div class="flex flex-wrap items-end justify-between gap-5">
            <div>
                <p class="font-mono text-xs uppercase tracking-[.2em] text-primary">"EUFRASIO / FIELDNOTES"</p>
                <h1 class="mt-3 text-4xl tracking-tight">"The notebook"</h1>
                <p class="mt-2 opacity-60">"Find a note, open it, read it all the way through."</p>
            </div>
            <a class="bg-primary px-4 py-3 font-mono text-sm text-white" href="/notes/new">"Write a note +"</a>
        </div>
        <div class="mt-8 max-w-xl">
            <input
                class="w-full border border-black/20 bg-white/40 px-4 py-3"
                type="search"
                placeholder="Search title, topic, or body..."
                :value=$(query.get())
                @input=$(|event: Event| query.set(event.target.value))
            >
        </div>
        <section class="mt-6 border-t border-black/15 pt-6">
            note_results(query: $(query.get()))
        </section>
    })
}

#[shard]
async fn note_results(cx: &Cx, query: String) -> Result<impl View> {
    let mut notes = data::list(cx).await?;
    notes.reverse();
    let query = query.trim().to_lowercase();
    let matches: Vec<_> = notes
        .into_iter()
        .filter(|note| {
            query.is_empty()
                || note.title.to_lowercase().contains(&query)
                || note.topic.to_lowercase().contains(&query)
                || note.body.to_lowercase().contains(&query)
        })
        .collect();

    Ok(view! {
        if matches.is_empty() {
            <p class="border border-dashed border-black/20 p-8 text-center opacity-65">
                if query.is_empty() {
                    "No notes yet. Publish the first fieldnote."
                } else {
                    "No notes match that search. Try another phrase."
                }
            </p>
        } else {
            <div class="grid gap-3 md:grid-cols-2">
                for note in matches {
                    let short_id = note.public_id.get(..8).unwrap_or(&note.public_id);
                    <a class="group border border-black/15 bg-white/35 p-5 transition hover:border-primary/60" href=(format!("/notes/{}", note.public_id))>
                        <div class="flex items-center justify-between gap-4">
                            <span class="font-mono text-xs uppercase tracking-widest text-primary">(&note.topic)</span>
                            <span class="font-mono text-[10px] opacity-40" title=(note.public_id.as_str())>"ID / " (short_id)</span>
                        </div>
                        <h2 class="mt-4 text-2xl leading-tight">(&note.title)</h2>
                        <p class="mt-5 font-mono text-xs text-primary opacity-0 transition group-hover:opacity-100">"OPEN FULL ENTRY →"</p>
                    </a>
                }
            </div>
        }
    })
}
