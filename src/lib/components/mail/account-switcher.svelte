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

  const selectedAccount = $derived(
    selectedAccountId ? accounts.find((a: Account) => a.id === selectedAccountId) : null
  );

  function selectAccount(accountId: string | null) {
    selectedAccountId = accountId;
    isOpen = false;
  }
</script>

{#if isCollapsed}
  <div class="flex justify-center p-2">
    <button
      class="flex h-9 w-9 items-center justify-center rounded-lg border bg-background text-sm font-medium hover:bg-accent"
      onclick={() => (isOpen = !isOpen)}
    >
      {#if selectedAccount}
        <div class={cn("h-3 w-3 rounded-full", selectedAccount.color)}></div>
      {:else}
        <div class="h-3 w-3 rounded-full bg-muted-foreground"></div>
      {/if}
    </button>
  </div>
{:else}
  <div class="relative p-2">
    <button
      class="flex w-full items-center justify-between rounded-lg border bg-background px-3 py-2 text-sm hover:bg-accent"
      onclick={() => (isOpen = !isOpen)}
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
      <div class="absolute left-2 right-2 z-50 mt-1 rounded-lg border bg-popover shadow-lg">
        <div class="p-1">
          <button
            class={cn(
              "flex w-full items-center gap-2 rounded px-2 py-1.5 text-sm hover:bg-accent",
              !selectedAccountId && "bg-accent"
            )}
            onclick={() => selectAccount(null)}
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
