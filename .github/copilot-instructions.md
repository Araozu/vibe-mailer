# Copilot Instructions for vibe-mailer

## Project Overview

This is a **Tauri + SvelteKit + TypeScript** application for managing email templates and sending emails.

- **Frontend**: Svelte 5 with Runes (SvelteKit)
- **Backend**: Rust (Tauri)
- **Styling**: Tailwind CSS v4 + shadcn-svelte components
- **Package Manager**: pnpm

## Development Commands

```bash
# Install dependencies
pnpm install

# Start development server
pnpm run dev

# Build the project
pnpm run build

# Type-check Svelte code
pnpm run check

# Watch mode for type checking
pnpm run check:watch

# Run Tauri development
pnpm tauri dev

# Build Tauri app
pnpm tauri build
```

## Project Structure

```
├── src/                  # Frontend SvelteKit application
│   ├── lib/             # Reusable Svelte components and utilities
│   ├── routes/          # SvelteKit routes
│   └── app.html         # HTML template
├── src-tauri/           # Rust backend
│   ├── src/             # Rust source code
│   ├── capabilities/    # Tauri capabilities configuration
│   ├── icons/           # Application icons
│   └── Cargo.toml       # Rust dependencies
├── static/              # Static assets
└── components.json      # shadcn-svelte configuration
```

## Coding Guidelines

### Frontend (Svelte + TypeScript)

1. **Svelte 5 Runes**: Use Svelte 5 syntax with runes (`$state`, `$derived`, `$effect`, etc.)
2. **TypeScript**: Always use TypeScript with proper type annotations
3. **Component Library**: Prefer **shadcn-svelte** components first
   - Install components: `pnpm dlx shadcn-svelte@latest add <component> -y`
   - Use shadcn components, variants, and utility patterns
4. **Styling**: 
   - Use **Tailwind CSS** classes for styling
   - Only use custom CSS when shadcn and Tailwind are insufficient
   - Follow shadcn-svelte style patterns
5. **Code Quality**: 
   - Run `pnpm run check` after making UI changes
   - Ensure no TypeScript errors before committing

### Backend (Rust)

1. **Framework**: Tauri 2.x
2. **Code Style**: Follow Rust standard conventions
3. **Error Handling**: Use proper Result types and error propagation
4. **Dependencies**: Keep dependencies minimal and well-maintained

## Best Practices

1. **Testing**: Always run `pnpm run check` before finalizing changes
2. **Components**: Reuse existing components from `src/lib` when possible
3. **Minimal Changes**: Make the smallest possible changes to achieve the goal
4. **Type Safety**: Maintain strong typing throughout the codebase
5. **Accessibility**: Ensure UI components are accessible (shadcn-svelte handles this by default)

## Important Notes

- **UI Focus**: Primary development focus is on the UI layer
- **shadcn-svelte First**: Always check if shadcn-svelte has a component before building custom UI
- **Tailwind v4**: This project uses Tailwind CSS v4 (check syntax compatibility)
- **SvelteKit**: Follow SvelteKit conventions for routing and data loading
- **Tauri Integration**: Be mindful of Tauri-specific APIs when working with system features

## Common Patterns

### Adding a New UI Component

1. Check if shadcn-svelte has the component: `pnpm dlx shadcn-svelte@latest add <component> -y`
2. Import and use in your Svelte file
3. Customize using Tailwind classes and shadcn variants
4. Run `pnpm run check` to verify

### Working with Svelte 5 Runes

```typescript
<script lang="ts">
  // State
  let count = $state(0);
  
  // Derived state
  let doubled = $derived(count * 2);
  
  // Effects
  $effect(() => {
    console.log(`Count is now ${count}`);
  });
</script>
```

### Tauri Commands (Rust to Frontend)

1. Define command in `src-tauri/src/lib.rs` or `main.rs`
2. Invoke from frontend using `@tauri-apps/api`

## Code Review Checklist

Before submitting changes:
- [ ] TypeScript types are correct (`pnpm run check` passes)
- [ ] UI follows shadcn-svelte patterns
- [ ] Tailwind classes are used appropriately
- [ ] No console errors in development
- [ ] Code follows existing project conventions
- [ ] Minimal changes approach followed
