<script lang="ts">
  import { cn } from "$lib/utils.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import type { Account } from "./data.js";
  import { ChevronDown, Check } from "@lucide/svelte";

  let { accounts, selectedAccountId = $bindable(), isCollapsed } = $props<{
    accounts: Account[];
    selectedAccountId: string | null;
    isCollapsed: boolean;
  }>();

  let isOpen = $state(false);
  let dropdownRef: HTMLDivElement | null = $state(null);

  const selectedAccount = $derived(
    selectedAccountId ? accounts.find((a: Account) => a.id === selectedAccountId) : null
  );

  function selectAccount(accountId: string | null) {
    selectedAccountId = accountId;
    isOpen = false;
  }

  function toggleDropdown() {
    isOpen = !isOpen;
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      isOpen = false;
    } else if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      toggleDropdown();
    }
  }

  function handleClickOutside(event: MouseEvent) {
    if (dropdownRef && !dropdownRef.contains(event.target as Node)) {
      isOpen = false;
    }
  }

  $effect(() => {
    if (isOpen) {
      document.addEventListener('click', handleClickOutside);
      return () => document.removeEventListener('click', handleClickOutside);
    }
  });
</script>

{#if isCollapsed}
  <div class="flex justify-center p-2">
    <button
      class="flex h-9 w-9 items-center justify-center rounded-lg border bg-background text-sm font-medium hover:bg-accent"
      onclick={toggleDropdown}
      onkeydown={handleKeydown}
      aria-label="Select account"
      aria-expanded={isOpen}
    >
      {#if selectedAccount}
        <div class={cn("h-3 w-3 rounded-full", selectedAccount.color)}></div>
      {:else}
        <div class="h-3 w-3 rounded-full bg-muted-foreground"></div>
      {/if}
    </button>
  </div>
{:else}
  <div class="relative p-2" bind:this={dropdownRef}>
    <button
      class="flex w-full items-center justify-between rounded-lg border bg-background px-3 py-2 text-sm hover:bg-accent"
      onclick={toggleDropdown}
      onkeydown={handleKeydown}
      aria-label="Select account"
      aria-expanded={isOpen}
      aria-haspopup="listbox"
    >
      <div class="flex items-center gap-2">
        {#if selectedAccount}
          <div class={cn("h-2 w-2 rounded-full", selectedAccount.color)}></div>
          <span class="font-medium">{selectedAccount.name}</span>
        {:else}
          <span class="font-medium">All Accounts</span>
        {/if}
      </div>
      <ChevronDown class={cn("h-4 w-4 transition-transform", isOpen && "rotate-180")} />
    </button>

    {#if isOpen}
      <div class="absolute left-2 right-2 z-50 mt-1 rounded-lg border bg-popover shadow-lg" role="listbox">
        <div class="p-1">
          <button
            class={cn(
              "flex w-full items-center gap-2 rounded px-2 py-1.5 text-sm hover:bg-accent",
              !selectedAccountId && "bg-accent"
            )}
            onclick={() => selectAccount(null)}
            role="option"
            aria-selected={!selectedAccountId}
          >
            {#if !selectedAccountId}
              <Check class="h-4 w-4" />
            {:else}
              <div class="h-4 w-4"></div>
            {/if}
            <span class="flex-1 text-left">All Accounts</span>
            <Badge variant="secondary">{accounts.length}</Badge>
          </button>
          {#each accounts as account}
            <button
              class={cn(
                "flex w-full items-center gap-2 rounded px-2 py-1.5 text-sm hover:bg-accent",
                selectedAccountId === account.id && "bg-accent"
              )}
              onclick={() => selectAccount(account.id)}
              role="option"
              aria-selected={selectedAccountId === account.id}
            >
              {#if selectedAccountId === account.id}
                <Check class="h-4 w-4" />
              {:else}
                <div class="h-4 w-4"></div>
              {/if}
              <div class={cn("h-2 w-2 rounded-full", account.color)}></div>
              <span class="flex-1 text-left">{account.name}</span>
            </button>
          {/each}
        </div>
      </div>
    {/if}
  </div>
{/if}
