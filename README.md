# Claude Code Session Viewer

A single-file HTML tool for browsing and searching [Claude Code](https://claude.ai/code) session JSONL transcripts.

## Features

- **File System Access API** — remembers your last opened file via IndexedDB; one-click "Reload Last" on subsequent visits
- **Dark/light theme** — follows system preference, persisted to `localStorage`
- **Search** — full-text search across all record fields with highlighted matches
- **Role filters** — toggle visibility by role: User, Assistant, Tool calls, Tool results, Attachments, Metadata, Thinking
- **Turn navigation** — auto-detected conversation turns with a collapsible sidebar outline
- **Token usage stats** — deduplicated by `requestId`: input tokens, cache reads/creates, output tokens
- **Markdown rendering** — headings, lists, task lists, blockquotes, tables, code blocks, inline code, links, images
- **Tool call/result display** — formatted cards with input summaries, expandable JSON details, collapsible outputs
- **Copy card content** — one-click copy of any event's text content to clipboard
- **Download filtered JSONL** — export the currently visible subset as a new `.jsonl` file
- **URL parameter support** — pass `?src=<url>` to load a remote JSONL file
- **Responsive layout** — works on desktop and mobile

## Usage

Open `claude-session-viewer.html` in a browser (Chrome recommended for File System Access API support).

Click **Select JSONL** and navigate into `~/.claude/projects/` to pick a session JSONL file. The viewer will remember your selection so **Reload Last** works on the next visit.

Alternatively, serve the file and pass a `?src=` parameter:

```
http://localhost:8080/claude-session-viewer.html?src=/path/to/session.jsonl
```

## Browser Support

File System Access API (`showOpenFilePicker`) is currently supported in Chromium-based browsers (Chrome, Edge, Opera). In other browsers the standard file picker fallback is used — you can still open files, but the "Reload Last" feature won't persist across sessions.

## License

MIT
