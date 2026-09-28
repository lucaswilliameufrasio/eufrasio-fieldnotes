import { Editor } from "https://esm.sh/@tiptap/core@2.11.5";
import StarterKit from "https://esm.sh/@tiptap/starter-kit@2.11.5";

for (const form of document.querySelectorAll("form")) {
  const source = form.querySelector("[data-editor-source]");
  const surface = form.querySelector("[data-editor-surface]");
  const toolbar = form.querySelector("[data-editor-toolbar]");
  if (!source || !surface || !toolbar) continue;

  const editor = new Editor({
    element: surface,
    extensions: [StarterKit],
    content: source.value || "<p></p>",
    editorProps: {
      attributes: {
        class: "ProseMirror focus:outline-none",
        "aria-label": "Fieldnote rich text editor",
      },
    },
    onUpdate({ editor: currentEditor }) {
      source.value = currentEditor.getHTML();
    },
  });

  source.value = editor.getHTML();
  source.hidden = true;
  surface.classList.remove("hidden");

  const commands = {
    bold: () => editor.chain().focus().toggleBold().run(),
    italic: () => editor.chain().focus().toggleItalic().run(),
    heading1: () => editor.chain().focus().toggleHeading({ level: 1 }).run(),
    heading2: () => editor.chain().focus().toggleHeading({ level: 2 }).run(),
    bulletList: () => editor.chain().focus().toggleBulletList().run(),
    blockquote: () => editor.chain().focus().toggleBlockquote().run(),
    codeBlock: () => editor.chain().focus().toggleCodeBlock().run(),
    undo: () => editor.chain().focus().undo().run(),
    redo: () => editor.chain().focus().redo().run(),
  };

  const refreshToolbar = () => {
    for (const button of toolbar.querySelectorAll("[data-command]")) {
      const command = button.dataset.command;
      const active = {
        bold: () => editor.isActive("bold"),
        italic: () => editor.isActive("italic"),
        heading1: () => editor.isActive("heading", { level: 1 }),
        heading2: () => editor.isActive("heading", { level: 2 }),
        bulletList: () => editor.isActive("bulletList"),
        blockquote: () => editor.isActive("blockquote"),
        codeBlock: () => editor.isActive("codeBlock"),
      }[command];
      button.classList.toggle("is-active", active?.() ?? false);
      button.setAttribute("aria-pressed", String(active?.() ?? false));
    }
  };

  toolbar.addEventListener("mousedown", (event) => event.preventDefault());
  toolbar.addEventListener("click", (event) => {
    const button = event.target.closest("[data-command]");
    if (button) commands[button.dataset.command]?.();
  });
  editor.on("selectionUpdate", refreshToolbar);
  editor.on("transaction", refreshToolbar);
  refreshToolbar();
  form.addEventListener("submit", () => {
    source.value = editor.getHTML();
  });
}
