# Coding Agent Session Viewer

A macOS app for browsing and searching session JSONL transcripts from AI coding agents. Currently supports:

- [Claude Code](https://claude.ai/code) — session files under `~/.claude/projects/`
- [Codex CLI](https://github.com/openai/codex) — rollout files under `~/.codex/sessions/`

The app auto-detects the format when you open a file — no manual switching needed.

## Features

- **Auto format detection** — opens both Claude Code and Codex CLI session files, automatically identifying the format and rendering accordingly
- **Dark/light theme** — follows system preference, persisted to `localStorage`
- **Search** — full-text search across all record fields with highlighted matches
- **Role filters** — toggle visibility: User, Assistant, Tool calls, Tool results, Attachments, Metadata, Thinking
- **Turn navigation** — auto-detected conversation turns with a collapsible sidebar outline
- **Token usage stats** — Claude Code: deduplicated by `requestId`; Codex CLI: summed per turn from `token_count` events
- **Markdown rendering** — headings, lists, task lists, blockquotes, tables, code blocks, inline code, links, images. Unicode box-drawing tables (used by Codex CLI terminal output) are preserved as monospaced blocks
- **Tool call/result display** — formatted cards with input summaries, expandable JSON details, collapsible outputs. Codex CLI's `exec_command`, `apply_patch`, `update_plan`, `spawn_agent` and other tools are supported
- **Patch apply rendering** — Codex CLI `patch_apply_end` events show changed files and unified diffs
- **Commentary filtering** — Codex CLI streaming commentary fragments are hidden by default; only final answers are rendered, keeping markdown intact
- **Copy card content** — one-click copy of any event's text content to clipboard
- **Download filtered JSONL** — export the currently visible subset as a new `.jsonl` file
- **Window state persistence** — remembers window size and position across launches
- **Responsive layout** — works on desktop and mobile

## macOS App

Built with [Tauri v2](https://tauri.app). A native macOS app with native file dialogs, persistent file memory, and window state restoration.

### Build from source

```bash
cd tauri-app
npm install
npx tauri build
```

The `.app` and `.dmg` will be under `tauri-app/src-tauri/target/release/bundle/`.

### Usage

1. Launch the app
2. Click **Select JSONL** and pick a session file from `~/.claude/projects/` or `~/.codex/sessions/` (use **Cmd+Shift+.** in the file dialog to show hidden directories)
3. The app auto-detects the format and renders the session

### Sharing

Send the `.dmg` to colleagues. On first launch, right-click the `.app` → **Open** to bypass Gatekeeper. No Apple Developer Program needed — the app is ad-hoc signed.

## Supported session formats

### Claude Code

JSONL files under `~/.claude/projects/<project>/<uuid>.jsonl`. Each line is a JSON object with a top-level `message` field and `uuid`. Record types include `user`, `assistant`, `system`, `attachment`, `ai-title`, and metadata types (`mode`, `file-history-snapshot`, etc.).

### Codex CLI

JSONL rollout files under `~/.codex/sessions/YYYY/MM/DD/rollout-<timestamp>-<uuid>.jsonl`. Each line is a JSON object with a top-level `payload` field. Top-level types include `session_meta`, `turn_context`, `response_item`, `event_msg`, and `compacted`. The `response_item` payload types include `message`, `function_call`, `function_call_output`, `custom_tool_call`, `custom_tool_call_output`, `reasoning`, `tool_search_call`, and `tool_search_output`. The `event_msg` payload types include `user_message`, `agent_message`, `task_started`, `task_complete`, `token_count`, `patch_apply_end`, and `context_compacted`.

## License

MIT
