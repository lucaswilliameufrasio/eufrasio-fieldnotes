# Eufrasio Fieldnotes

A public notebook for experiments in software and design, with separate overview, searchable notebook, create, full-note reading, and edit screens. Notes are persisted in `fieldnotes.sqlite` with Toasty and addressed by UUIDv7 public IDs. Browser-side search uses a Topcoat signal and reruns a server-rendered shard against the database. Create/edit screens use Tiptap for rich text; submitted and previously stored HTML is sanitized server-side before it is rendered.

## Run

Install the Topcoat CLI once, then start the app:

```sh
cargo install topcoat-cli
topcoat dev
```

Open <http://127.0.0.1:3000>. Browse `/notes`, create at `/notes/new`, click a result to read the full entry, or edit it from its detail page. A first run inserts one formatted example note into SQLite; new notes and edits use the same real persistence flow. The Tiptap modules load from esm.sh, so a browser needs internet access for the editor toolbar; the server-side note workflow remains Rust + Topcoat + Toasty.
