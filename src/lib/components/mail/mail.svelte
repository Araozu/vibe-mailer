<script lang="ts">
  import { cn } from "$lib/utils.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { ResizablePane, ResizablePaneGroup, ResizableHandle } from "$lib/components/ui/resizable/index.js";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import { Tabs, TabsContent, TabsList, TabsTrigger } from "$lib/components/ui/tabs/index.js";
  import { Search, Inbox, File, Send, Archive, Trash2, Users, AlertCircle, MessagesSquare, ShoppingCart, Tag } from "@lucide/svelte";
  import MailList from "./mail-list.svelte";
  import MailDisplay from "./mail-display.svelte";
  import Nav from "./nav.svelte";
  import { mails } from "./data.js";

  let isCollapsed = $state(false);
  let selectedMailId = $state(mails[0].id);
  let search = $state("");

  const selectedMail = $derived(
    mails.find((item) => item.id === selectedMailId) || null
  );

  const filteredMails = $derived(
    mails.filter((item) =>
      item.subject.toLowerCase().includes(search.toLowerCase()) ||
      item.name.toLowerCase().includes(search.toLowerCase())
    )
  );
</script>

<div class="h-screen w-full overflow-hidden border-t">
  <ResizablePaneGroup direction="horizontal" class="h-full items-stretch">
    <ResizablePane
      defaultSize={20}
      collapsedSize={4}
      collapsible={true}
      minSize={15}
      maxSize={20}
      onCollapse={() => (isCollapsed = true)}
      onExpand={() => (isCollapsed = false)}
      class={cn(isCollapsed && "min-w-12.5 transition-all duration-300 ease-in-out")}
    >
      <div class={cn("flex h-13 items-center justify-center", isCollapsed ? "h-13" : "px-2")}>
        {#if !isCollapsed}
          <div class="flex items-center gap-2 font-semibold">
             Mail App
          </div>
        {/if}
      </div>
      <Separator />
      <Nav
        {isCollapsed}
        links={[
          { title: "Inbox", label: "128", icon: Inbox, variant: "default" },
          { title: "Drafts", label: "9", icon: File, variant: "ghost" },
          { title: "Sent", label: "", icon: Send, variant: "ghost" },
          { title: "Junk", label: "23", icon: Archive, variant: "ghost" },
          { title: "Trash", label: "", icon: Trash2, variant: "ghost" },
          { title: "Archive", label: "", icon: Archive, variant: "ghost" },
        ]}
      />
      <Separator />
      <Nav
        {isCollapsed}
        links={[
          { title: "Social", label: "972", icon: Users, variant: "ghost" },
          { title: "Updates", label: "342", icon: AlertCircle, variant: "ghost" },
          { title: "Forums", label: "128", icon: MessagesSquare, variant: "ghost" },
          { title: "Shopping", label: "8", icon: ShoppingCart, variant: "ghost" },
          { title: "Promotions", label: "21", icon: Tag, variant: "ghost" },
        ]}
      />
    </ResizablePane>
    <ResizableHandle withHandle />
    <ResizablePane defaultSize={35} minSize={30}>
      <Tabs value="all">
        <div class="flex items-center px-4 py-2">
          <h1 class="text-xl font-bold">Inbox</h1>
          <TabsList class="ml-auto">
            <TabsTrigger value="all" class="text-zinc-600 dark:text-zinc-200">All mail</TabsTrigger>
            <TabsTrigger value="unread" class="text-zinc-600 dark:text-zinc-200">Unread</TabsTrigger>
          </TabsList>
        </div>
        <Separator />
        <div class="bg-background/95 p-4 backdrop-blur supports-backdrop-filter:bg-background/60">
          <form>
            <div class="relative">
              <Search class="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
              <Input placeholder="Search" class="pl-8" bind:value={search} />
            </div>
          </form>
        </div>
        <TabsContent value="all" class="m-0">
          <MailList items={filteredMails} bind:selectedMailId />
        </TabsContent>
        <TabsContent value="unread" class="m-0">
          <MailList
            items={filteredMails.filter((mail) => !mail.read)}
            bind:selectedMailId
          />
        </TabsContent>
      </Tabs>
    </ResizablePane>
    <ResizableHandle withHandle />
    <ResizablePane defaultSize={45}>
        <MailDisplay mail={selectedMail} />
    </ResizablePane>
  </ResizablePaneGroup>
</div>
