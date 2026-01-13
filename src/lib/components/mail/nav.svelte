<script lang="ts">
  import { cn } from "$lib/utils.js";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import type { Component } from "svelte";

  let { links, isCollapsed } = $props<{
    links: {
      title: string;
      label?: string;
      icon: Component<any>;
      variant: "default" | "ghost";
    }[];
    isCollapsed: boolean;
  }>();
</script>

<div
  data-collapsed={isCollapsed}
  class="group flex flex-col gap-4 py-2 data-[collapsed=true]:py-2"
>
  <nav
    class="grid gap-1 px-2 group-data-[collapsed=true]:justify-center group-data-[collapsed=true]:px-2"
  >
    {#each links as link}
      {@const Icon = link.icon}
      {#if isCollapsed}
        <Icon
          class={cn(
            buttonVariants({ variant: link.variant, size: "icon" }),
            "h-9 w-9",
            link.variant === "default" &&
              "dark:bg-muted dark:text-muted-foreground dark:hover:bg-muted dark:hover:text-white"
          )}
        />
      {:else}
        <a
          href="/"
          class={cn(
            buttonVariants({ variant: link.variant, size: "sm" }),
            link.variant === "default" &&
              "dark:bg-muted dark:text-white dark:hover:bg-muted dark:hover:text-white",
            "justify-start"
          )}
        >
          <Icon class="mr-2 h-4 w-4" />
          {link.title}
          {#if link.label}
            <span
              class={cn(
                "ml-auto",
                link.variant === "default" && "text-background dark:text-white"
              )}
            >
              {link.label}
            </span>
          {/if}
        </a>
      {/if}
    {/each}
  </nav>
</div>
