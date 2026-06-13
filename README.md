# Claude Code Session Viewer

A tool for browsing and searching [Claude Code](https://claude.ai/code) session JSONL transcripts.

Two flavors available:

| | Standalone HTML | macOS App |
|---|---|---|
| **Directory** | `claude-session-viewer.html` | `tauri-app/` |
| **Deployment** | Single file, open in browser | Native `.app` bundle |
| **File picker** | Browser file dialog | Native macOS dialog (Cmd+Shift+. for hidden dirs) |
| **Default directory** | — | `~/.claude/projects` |
| **Last file memory** | IndexedDB (Chrome only) | Tauri Store (always works) |
| **Sharing** | Send the `.html` file | Send the `.dmg` |

## Features

- **Dark/light theme** — follows system preference, persisted to `localStorage`
- **Search** — full-text search across all record fields with highlighted matches
- **Role filters** — toggle visibility: User, Assistant, Tool calls, Tool results, Attachments, Metadata, Thinking
- **Turn navigation** — auto-detected conversation turns with a collapsible sidebar outline
- **Token usage stats** — deduplicated by `requestId`: input tokens, cache reads/creates, output tokens
- **Markdown rendering** — headings, lists, task lists, blockquotes, tables, code blocks, inline code, links, images
- **Tool call/result display** — formatted cards with input summaries, expandable JSON details, collapsible outputs
- **Copy card content** — one-click copy of any event's text content to clipboard
- **Download filtered JSONL** — export the currently visible subset as a new `.jsonl` file
- **URL parameter support** — pass `?src=<url>` to load a remote JSONL file (`?src=` works in the HTML version; native file open in the app)
- **Responsive layout** — works on desktop and mobile

## Standalone HTML

Open `claude-session-viewer.html` in a browser. Chrome recommended — the File System Access API enables "Reload Last" to remember your file across sessions.

Click **Select JSONL** and navigate into `~/.claude/projects/` to pick a session file. On macOS use **Cmd+Shift+.** in the file dialog to show hidden directories.

Alternatively, serve the file and pass a `?src=` parameter:

```
http://localhost:8080/claude-session-viewer.html?src=/path/to/session.jsonl
```

## macOS App

Built with [Tauri v2](https://tauri.app). A native macOS app with the same interface, using native file dialogs and persistent file memory.

### Build from source

```bash
cd tauri-app
npm install
npx tauri build
```

The `.app` and `.dmg` will be under `tauri-app/src-tauri/target/release/bundle/`.

### Sharing

Send the `.dmg` to colleagues. On first launch, right-click the `.app` → **Open** to bypass Gatekeeper. No Apple Developer Program needed — the app is ad-hoc signed.

## License

MIT
