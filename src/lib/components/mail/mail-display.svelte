<script lang="ts">
  import { Avatar, AvatarFallback, AvatarImage } from "$lib/components/ui/avatar/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import type { Mail, Account } from "./data.js";
  import { Archive, ArchiveRestore, MoreVertical, Reply, ReplyAll, Forward, Trash2 } from "@lucide/svelte";
  import { cn } from "$lib/utils.js";

  let { mail, accounts } = $props<{ mail: Mail | null; accounts: Account[] }>();

  const recipientAccount = $derived(
    mail ? accounts.find((a: Account) => a.id === mail.accountId) : null
  );
</script>

<div class="flex h-full flex-col">
  <div class="flex items-center p-2">
    <div class="flex items-center gap-2">
        <Button variant="ghost" size="icon" disabled={!mail}>
            <Archive class="h-4 w-4" />
        </Button>
        <Button variant="ghost" size="icon" disabled={!mail}>
            <ArchiveRestore class="h-4 w-4" />
        </Button>
        <Button variant="ghost" size="icon" disabled={!mail}>
            <Trash2 class="h-4 w-4" />
        </Button>
    </div>
    <div class="ml-auto flex items-center gap-2">
        <Button variant="ghost" size="icon" disabled={!mail}>
            <Reply class="h-4 w-4" />
        </Button>
        <Button variant="ghost" size="icon" disabled={!mail}>
            <ReplyAll class="h-4 w-4" />
        </Button>
        <Button variant="ghost" size="icon" disabled={!mail}>
            <Forward class="h-4 w-4" />
        </Button>
        <Separator orientation="vertical" class="mx-1 h-6" />
        <Button variant="ghost" size="icon" disabled={!mail}>
            <MoreVertical class="h-4 w-4" />
        </Button>
    </div>
  </div>
  <Separator />
  {#if mail}
    <div class="flex flex-1 flex-col">
      <div class="flex items-start p-4">
        <div class="flex items-start gap-4 text-sm">
          <Avatar>
            <AvatarFallback>
              {mail.name
                .split(" ")
                .map((chunk: string) => chunk[0])
                .join("")}
            </AvatarFallback>
          </Avatar>
          <div class="grid gap-1">
            <div class="font-semibold">{mail.name}</div>
            <div class="line-clamp-1 text-xs">{mail.subject}</div>
            <div class="line-clamp-1 text-xs">
              <span class="font-medium">Reply-To:</span> {mail.email}
            </div>
            {#if recipientAccount}
              <div class="flex items-center gap-1 text-xs">
                <span class="font-medium">To:</span>
                <Badge variant="secondary" class="gap-1">
                  <div class={cn("h-1.5 w-1.5 rounded-full", recipientAccount.color)}></div>
                  {recipientAccount.email}
                </Badge>
              </div>
            {/if}
          </div>
        </div>
        {#if mail.date}
          <div class="ml-auto text-xs text-muted-foreground">
            {new Date(mail.date).toLocaleString()}
          </div>
        {/if}
      </div>
      <Separator />
      <div class="flex-1 whitespace-pre-wrap p-4 text-sm">
        {mail.text}
      </div>
    </div>
  {:else}
    <div class="p-8 text-center text-muted-foreground">
      No message selected
    </div>
  {/if}
</div>
