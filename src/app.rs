pub(crate) mod notes;

use toasty::Db;
use topcoat::{
    Result,
    asset::{Asset, AssetBundle, RouterBuilderAssetExt, asset},
    context::try_request_context,
    router::{
        Body, LayerFn, LayerFuture, Next, Router, RouterBuilderDiscoverExt, Slot, StatusCode,
        error::{
            BadRequestError, ContentTooLargeError, ForbiddenError, MethodNotAllowedError,
            NotFoundError, ServiceUnavailableError, TooManyRequestsError, UnauthorizedError,
            rewrite,
        },
        layout, module_router, not_found, page,
    },
    runtime::RouterBuilderRuntimeExt,
    tailwind,
    view::{View, error_boundary, view},
};

const EDITOR_SCRIPT: Asset = asset!("./app/notes/editor.js");

not_found!("/");

pub(crate) fn router(db: Db) -> Router {
    module_router!()
        .layer(LayerFn::new(None::<&str>, error_layer))
        .discover()
        .assets(AssetBundle::load().unwrap())
        .app_context(db)
        .runtime()
        .build()
}

#[derive(Clone, Copy)]
struct ErrorPage {
    status: StatusCode,
    code: &'static str,
    title: &'static str,
    message: &'static str,
}

const NOT_FOUND_PAGE: ErrorPage = ErrorPage {
    status: StatusCode::NOT_FOUND,
    code: "404",
    title: "This leaf isn't in the notebook.",
    message: "The note may have moved, or this link may use an older numeric ID. Fieldnotes links now use UUIDv7.",
};

fn describe_error(error: &topcoat::Error) -> ErrorPage {
    if error.downcast_ref::<NotFoundError>().is_some() {
        NOT_FOUND_PAGE
    } else if error.downcast_ref::<BadRequestError>().is_some() {
        ErrorPage {
            status: StatusCode::BAD_REQUEST,
            code: "400",
            title: "This entry could not be read.",
            message: "Check the note fields and try submitting it again.",
        }
    } else if error.downcast_ref::<UnauthorizedError>().is_some() {
        ErrorPage {
            status: StatusCode::UNAUTHORIZED,
            code: "401",
            title: "This page needs a reader session.",
            message: "Sign in and follow the note link again.",
        }
    } else if error.downcast_ref::<ForbiddenError>().is_some() {
        ErrorPage {
            status: StatusCode::FORBIDDEN,
            code: "403",
            title: "This page is not yours to change.",
            message: "The request is not permitted.",
        }
    } else if error.downcast_ref::<MethodNotAllowedError>().is_some() {
        ErrorPage {
            status: StatusCode::METHOD_NOT_ALLOWED,
            code: "405",
            title: "That action doesn't belong on this page.",
            message: "Return to the notebook and choose one of its available actions.",
        }
    } else if error.downcast_ref::<ContentTooLargeError>().is_some() {
        ErrorPage {
            status: StatusCode::PAYLOAD_TOO_LARGE,
            code: "413",
            title: "This fieldnote is too long.",
            message: "Trim the entry and try saving it again.",
        }
    } else if error.downcast_ref::<TooManyRequestsError>().is_some() {
        ErrorPage {
            status: StatusCode::TOO_MANY_REQUESTS,
            code: "429",
            title: "Too many pages turned at once.",
            message: "Pause for a moment, then try again.",
        }
    } else if error.downcast_ref::<ServiceUnavailableError>().is_some() {
        ErrorPage {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "503",
            title: "The notebook is taking a short break.",
            message: "Your saved entries are safe. Please try again shortly.",
        }
    } else {
        ErrorPage {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "500",
            title: "A page tore while we were reading it.",
            message: "An unexpected error occurred. Its technical details stay on the server.",
        }
    }
}

fn error_content(cx: &topcoat::context::Cx, error: ErrorPage) -> impl View {
    view! { cx =>
        (error.status)
        <section class="mx-auto max-w-3xl border border-black/15 bg-white/45 p-8 md:p-12">
            <p class="font-mono text-xs uppercase tracking-[.2em] text-primary">"EUFRASIO / FIELDNOTES · " (error.code)</p>
            <h1 class="mt-6 text-4xl leading-tight tracking-[-.04em] md:text-6xl">(error.title)</h1>
            <p class="mt-5 max-w-2xl text-lg opacity-65">(error.message)</p>
            <div class="mt-8 flex flex-wrap gap-3">
                <a class="bg-primary px-5 py-3 font-mono text-sm text-white" href="/">"Notebook home"</a>
                <a class="border border-black/20 px-5 py-3 font-mono text-sm" href="/notes">"Browse entries"</a>
            </div>
        </section>
    }
}

fn error_layer<'a>(cx: &'a topcoat::context::Cx, body: Body, next: Next<'a>) -> LayerFuture<'a> {
    Box::pin(async move {
        match next.run(cx, body).await {
            Ok(response) => Ok(response),
            Err(error) => {
                let page = describe_error(&error);
                Err(rewrite("/_errors", Body::empty())
                    .method(topcoat::router::Method::GET)
                    .with(page)
                    .into())
            }
        }
    })
}

#[page("/_errors")]
async fn error_page(cx: &topcoat::context::Cx) -> Result<impl View> {
    let page = try_request_context::<ErrorPage>(cx)
        .copied()
        .unwrap_or(NOT_FOUND_PAGE);
    Ok(error_content(cx, page))
}

#[layout]
async fn shell(cx: &topcoat::context::Cx, slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <meta name="theme-color" content="#f4f1e9">
                <title>"Eufrasio Fieldnotes"</title>
                topcoat::runtime::script()
                <script type="module" src=(EDITOR_SCRIPT)></script>
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
            </head>
            <body class="min-h-screen">
                <header class="border-b border-black/15">
                    <nav class="mx-auto flex max-w-5xl items-center justify-between px-6 py-5">
                        <a class="font-mono text-xs font-bold uppercase tracking-[.18em] text-primary" href="/">
                            "EUFRASIO / FIELDNOTES"
                        </a>
                        <div class="flex items-center gap-5 font-mono text-xs uppercase tracking-widest">
                            <a class="opacity-70 hover:text-primary" href="/notes">"Notebook"</a>
                            <a class="text-primary" href="/notes/new">"New note +"</a>
                        </div>
                    </nav>
                </header>
                <main class="mx-auto max-w-5xl px-6 py-12">
                    error_boundary(
                        fallback: |error| Ok(error_content(cx, describe_error(&error))),
                        (slot)
                    )
                </main>
                <footer class="mx-auto max-w-5xl border-t border-black/15 px-6 py-5 font-mono text-xs opacity-50">"A working notebook, built with Topcoat + Toasty."</footer>
            </body>
        </html>
    })
}

#[page]
async fn home(cx: &topcoat::context::Cx) -> Result<impl View> {
    let notes = crate::data::list(cx).await?;
    let total = notes.len();

    Ok(view! {
        <section class="grid gap-12 md:grid-cols-[1fr_280px]">
            <div>
                <p class="font-mono text-xs uppercase tracking-[.2em] text-primary">"NOTES FROM THE WORKBENCH"</p>
                <h1 class="mt-4 max-w-3xl text-5xl leading-[.98] tracking-[-.06em] md:text-7xl">
                    "Make a thing."<br>"Notice what happens."
                </h1>
                <p class="mt-6 max-w-2xl text-lg opacity-70">
                    "Experiments in software, design, and the curious space between them. Each note is a full entry, not a card-sized thought."
                </p>
                <div class="mt-8 flex gap-3">
                    <a class="bg-primary px-5 py-3 font-mono text-sm text-white" href="/notes/new">"Write a fieldnote →"</a>
                    <a class="border border-black/20 px-5 py-3 font-mono text-sm" href="/notes">"Browse notebook"</a>
                </div>
            </div>
            <aside class="h-fit border border-black/15 bg-white/40 p-6">
                <p class="font-mono text-xs uppercase tracking-widest text-primary">"On record"</p>
                <p class="mt-8 text-5xl">(total)</p>
                <p class="mt-2 text-sm opacity-60">"notes published"</p>
                <p class="mt-6 border-t border-black/10 pt-5 text-sm opacity-65">"Entries are saved to a local SQLite database with Toasty."</p>
            </aside>
        </section>
        <section class="mt-16 border-t border-black/15 pt-8">
            <div class="flex items-center justify-between gap-4">
                <h2 class="text-2xl">"Recently added"</h2>
                <a class="font-mono text-xs text-primary" href="/notes">"OPEN NOTEBOOK →"</a>
            </div>
            if notes.is_empty() {
                <p class="mt-6 border border-dashed border-black/20 p-8 text-center opacity-65">"Your notebook is empty. Start with one experiment."</p>
            } else {
                <div class="mt-5 grid gap-3 md:grid-cols-2">
                    for note in notes.iter().rev().take(4) {
                        <a class="border border-black/15 bg-white/35 p-5 hover:border-primary/60" href=(format!("/notes/{}", note.public_id))>
                            <p class="font-mono text-xs uppercase tracking-widest text-primary">(&note.topic)</p>
                            <h3 class="mt-3 text-xl">(&note.title)</h3>
                            <p class="mt-2 line-clamp-2 text-sm opacity-60">"Read the full note →"</p>
                        </a>
                    }
                </div>
            }
        </section>
    })
}

#[cfg(test)]
mod tests {
    use super::{NOT_FOUND_PAGE, describe_error};
    use topcoat::router::{StatusCode, error::bad_request};

    #[test]
    fn custom_error_mapping_keeps_status_and_does_not_expose_internal_messages() {
        let not_found: topcoat::Error = topcoat::router::error::not_found().into();
        assert_eq!(describe_error(&not_found).status, StatusCode::NOT_FOUND);
        assert_eq!(describe_error(&not_found).title, NOT_FOUND_PAGE.title);

        let bad_request: topcoat::Error = bad_request("private form detail").into();
        let page = describe_error(&bad_request);
        assert_eq!(page.status, StatusCode::BAD_REQUEST);
        assert!(!page.message.contains("private form detail"));

        let internal = topcoat::Error::msg("database password should not reach the page");
        let page = describe_error(&internal);
        assert_eq!(page.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(!page.message.contains("database password"));
    }
}
