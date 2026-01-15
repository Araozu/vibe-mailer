# Vibe Mailer — Roadmap

A power-user focused email client built with Tauri (Rust backend, Svelte frontend).

---

## Project Overview

| Aspect | Decision |
|--------|----------|
| **Framework** | Tauri v2 |
| **Backend** | Rust |
| **Frontend** | Svelte 5 (Runes) + shadcn-svelte |
| **Protocol** | IMAP only (no POP3) |
| **Target users** | Power users — many knobs, keyboard-driven (eventually) |
| **Platforms** | Linux first, Windows second, macOS maybe |

---

## Architecture Decisions

### Backend (Rust)

| Component | Choice | Notes |
|-----------|--------|-------|
| IMAP client | `async-imap` | Async, fits Tauri's model |
| TLS | Implicit TLS (port 993) | STARTTLS later if needed |
| Database | SQLite | Single file, metadata + email bodies |
| Email bodies | Stored in SQLite | Sanitized HTML, not raw MIME |
| Credentials | System keychain via `keyring` | Linux: libsecret, Windows: Credential Manager |
| Config path | XDG (`~/.config/vibe-mailer/`) | |
| Config format | YAML | |

### Frontend (Svelte)

| Component | Choice |
|-----------|--------|
| UI framework | shadcn-svelte |
| Styling | Tailwind CSS |
| Theme | Dark mode with system-follow + manual override |

---

## Phases

### Phase 0: MVP — Read-Only Single Account

**Goal:** Connect to an IMAP server, fetch and display the last 50 emails.

#### Features
- [ ] IMAP connection with username/password over TLS
- [ ] Fetch Inbox folder
- [ ] Cache last 50 emails in SQLite (metadata + sanitized body)
- [ ] Flat email list (no threading)
- [ ] Email viewer with sanitized HTML rendering
- [ ] Manual refresh
- [ ] Single account only
- [ ] Basic settings UI (server, port, username, password)

#### Non-goals for MVP
- No compose/reply/forward/delete
- No attachments
- No search
- No multiple accounts
- No background sync
- No notifications

---

### Phase 1: Essential Folders + Trash

- [ ] Fetch Trash folder
- [ ] Folder switcher in sidebar (Inbox / Trash)
- [ ] Delete emails (move to Trash)
- [ ] Empty Trash

---

### Phase 2: Compose & Reply

- [ ] Compose new email
- [ ] Reply / Reply All
- [ ] Forward
- [ ] Drafts folder
- [ ] Sent folder
- [ ] Move emails between folders

---

### Phase 3: Search & Threading

- [ ] Local search (SQLite FTS)
- [ ] Conversation threading (group by subject/references)
- [ ] IMAP server-side search

---

### Phase 4: Attachments

- [ ] Display attachment list in email viewer
- [ ] Download attachments
- [ ] Attach files when composing

---

### Phase 5: Multiple Accounts

- [ ] Add/remove multiple IMAP accounts
- [ ] Account switcher in sidebar
- [ ] Unified inbox (all accounts combined)
- [ ] Per-account settings

---

### Phase 6: OAuth

- [ ] OAuth2 flow for Gmail
- [ ] OAuth2 flow for Outlook
- [ ] Token refresh handling

---

### Phase 7: Background Sync & Notifications

- [ ] Background sync (configurable interval or IMAP IDLE)
- [ ] System tray with unread count
- [ ] Desktop notifications for new emails
- [ ] Minimize to tray option

---

### Phase 8: Power User Features

- [ ] Keyboard shortcuts (configurable)
- [ ] Quick actions / command palette
- [ ] Email filters / rules
- [ ] Custom folders
- [ ] Labels / tags
- [ ] Notification granularity per account/folder

---

## Open Questions / TBD

- [ ] UI layout inspiration (3-pane classic? something else?)
- [ ] Keyboard shortcut scheme (vim-like? standard? custom?)
- [ ] Exact sanitization rules for HTML emails
- [ ] How to handle inline images (block, proxy, allow?)

---

## Tech Stack Summary

```
┌─────────────────────────────────────────────┐
│                  Frontend                   │
│  Svelte 5 + shadcn-svelte + Tailwind CSS    │
└─────────────────┬───────────────────────────┘
                  │ Tauri IPC
┌─────────────────▼───────────────────────────┐
│                  Backend                    │
│  Rust + async-imap + rusqlite + keyring     │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│              IMAP Server                    │
│         (TLS, port 993)                     │
└─────────────────────────────────────────────┘
```

---

## References

- [Tauri v2 docs](https://v2.tauri.app/)
- [async-imap crate](https://crates.io/crates/async-imap)
- [keyring crate](https://crates.io/crates/keyring)
- [shadcn-svelte](https://shadcn-svelte.com/)
