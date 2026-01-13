
# Agent Instructions

## Context
- **Framework**: Tauri application.
- **Frontend**: Svelte with Runes.
- **Scope**: Work only on UI.

## UI Guidelines

- **shadcn-svelte** first: Always prefer shadcn components.
- Install: `pnpm dlx shadcn-svelte@latest add <component> -y`
- **Tailwind** second: Use manual tailwind classes only if shadcn is insufficient.
- Prefer shadcn styles, variants, and utility patterns over custom CSS.

## Quality

- After working in UI, run `pnpm run check` to check that Svelte code is good.
