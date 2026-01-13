<script lang="ts">
  import { cn } from "$lib/utils.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
  import type { Mail } from "./data.js";
  import { formatDistanceToNow } from "date-fns";

  let { items, selectedMailId = $bindable() } = $props<{
    items: Mail[];
    selectedMailId: string | null;
  }>();
</script>

<ScrollArea class="h-screen">
  <div class="flex flex-col gap-2 p-4 pt-0">
    {#each items as item (item.id)}
      <button
        class={cn(
          "flex flex-col items-start gap-2 rounded-lg border p-3 text-left text-sm transition-all hover:bg-accent",
          selectedMailId === item.id && "bg-muted"
        )}
        onclick={() => (selectedMailId = item.id)}
      >
        <div class="flex w-full flex-col gap-1">
          <div class="flex items-center">
            <div class="flex items-center gap-2">
              <div class="font-semibold">{item.name}</div>
              {#if !item.read}
                <span class="flex h-2 w-2 rounded-full bg-blue-600"></span>
              {/if}
            </div>
            <div
              class={cn(
                "ml-auto text-xs",
                selectedMailId === item.id
                  ? "text-foreground"
                  : "text-muted-foreground"
              )}
            >
              {formatDistanceToNow(new Date(item.date), { addSuffix: true })}
            </div>
          </div>
          <div class="text-xs font-medium">{item.subject}</div>
        </div>
        <div class="line-clamp-2 text-xs text-muted-foreground">
          {item.text.substring(0, 300)}
        </div>
        {#if item.labels.length}
          <div class="flex items-center gap-2">
            {#each item.labels as label}
              <Badge variant={label === "work" ? "default" : "outline"}>
                {label}
              </Badge>
            {/each}
          </div>
        {/if}
      </button>
    {/each}
  </div>
</ScrollArea>
