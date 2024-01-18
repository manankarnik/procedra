<script>
  import { User, Heart, MoreVertical, Pencil, Trash } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import * as Dialog from "$lib/components/ui/dialog";
  import * as Avatar from "$lib/components/ui/avatar";
  export let asset;
  export let manage = false;
  let open = false;
  let title;

  async function deleteAsset(id) {
    await fetch("/asset", { method: "DELETE", body: JSON.stringify({ id }) });
  }
</script>

<div class="animate-gradient p-[4px] hover:border-transparent hover:bg-gradient-to-r rounded-xl">
  <div class="h-full w-full rounded-lg bg-slate-100 p-4 dark:bg-slate-900">
    <svelte:element
      this={manage ? "div" : "button"}
      class="w-full"
      on:click={manage
        ? () => {}
        : () => (window.location.href = `/generate/${asset.type}?id=${asset.id}`)}
    >
      <div class="flex items-center justify-between">
        <div>
          <h2
            class="animate-gradient bg-gradient-to-r bg-clip-text text-start text-lg font-bold text-transparent md:text-xl"
          >
            {asset.title}
          </h2>
          {#if !manage}
            <div class="text-md flex items-center gap-2 pt-2">
              <Avatar.Root class="h-8 w-8">
                <Avatar.Image src={asset.user.image} alt="Profile" />
                <Avatar.Fallback>
                  <User />
                </Avatar.Fallback>
              </Avatar.Root>
              {asset.user.name}
            </div>
          {/if}
        </div>
        {#if manage}
          <DropdownMenu.Root>
            <DropdownMenu.Trigger>
              <Button variant="ghost" size="icon">
                <MoreVertical size={18} />
              </Button>
            </DropdownMenu.Trigger>
            <DropdownMenu.Content>
              <DropdownMenu.Item
                class="flex flex-row items-center gap-2"
                on:click={() =>
                  (window.location.href = `/manage/edit/${asset.type}?id=${asset.id}`)}
              >
                <Pencil size={20} />
                Edit Asset
              </DropdownMenu.Item>
              <DropdownMenu.Item
                class="flex flex-row items-center gap-2 text-destructive hover:bg-destructive"
                on:click={() => {
                  title = asset.title;
                  open = true;
                }}
              >
                <Trash size={20} />
                Delete Asset
              </DropdownMenu.Item>
            </DropdownMenu.Content>
          </DropdownMenu.Root>
        {/if}
      </div>
      <div>
        <hr />
        <img
          class="flex h-[200px] w-full items-center justify-center rounded-b-lg object-contain"
          src={asset.thumbnail}
          alt="Asset thumbnail"
        />
        <hr />
        <div class="flex gap-2">
          <Heart />
          1K
        </div>
        <div class="md:text-md pt-2 text-start text-muted-foreground">
          {#if asset.description}
            {asset.description}
          {:else}
            <p class="text-sm italic">No description provided</p>
          {/if}
        </div>
      </div>
    </svelte:element>
  </div>
</div>

<Dialog.Root {open}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Delete Asset</Dialog.Title>
      <Dialog.Description>
        Are you sure you want to <span class="font-bold text-destructive">DELETE</span> the asset
        <span class="font-bold text-primary">{title}</span>?
      </Dialog.Description>
    </Dialog.Header>
    <div class="flex justify-end gap-4">
      <Button variant="outline" on:click={() => (open = false)}>Cancel</Button>
      <Button variant="destructive" on:click={deleteAsset(asset.id)}>Delete</Button>
    </div>
  </Dialog.Content>
</Dialog.Root>
