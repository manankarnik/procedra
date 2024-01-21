<script>
  import { page } from "$app/stores";
  import { User, Heart, MoreVertical, Pencil, Trash, Loader2, CheckCircle2 } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import * as Dialog from "$lib/components/ui/dialog";
  import * as Avatar from "$lib/components/ui/avatar";
  import * as Tooltip from "$lib/components/ui/tooltip";
  import * as Alert from "$lib/components/ui/alert";

  export let asset;
  export let manage = false;
  let open = false;
  let title;
  let animate;
  let loading;
  let alert;

  async function deleteAsset(id) {
    loading = true;
    await fetch("/asset", { method: "DELETE", body: JSON.stringify({ id }) });
    loading = false;
    open = false;
    alert = true;
    setTimeout(() => {
      alert = false;
    }, 2000);
  }
  async function changeLikeStatus() {
    animate = true;
    setTimeout(() => {
      animate = false;
    }, 250);
    const response = await fetch("/like", {
      method: "POST",
      body: JSON.stringify({
        assetId: asset.id,
        userId: $page.data.userId
      })
    });
    asset.liked = (await response.json()).liked;
    asset.likes = asset.liked ? asset.likes + 1 : asset.likes - 1;
  }
</script>

<div class={`top-20 left-0 fixed flex h-20 w-full justify-center ${alert ? "" : "hidden"}`}>
  <Alert.Root variant="success" class="w-30 bg-slate-100 dark:bg-slate-900">
    <CheckCircle2 class="h-4 w-4 text-green-500" />
    <Alert.Title>Success</Alert.Title>
    <Alert.Description>Asset deleted successfully!</Alert.Description>
  </Alert.Root>
</div>
<div class="animate-gradient rounded-xl p-[4px] hover:border-transparent hover:bg-gradient-to-r">
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
        {:else if $page.data.userId}
          <Tooltip.Root>
            <Tooltip.Trigger class="flex items-center">
              <button
                class="rounded-md p-2 hover:bg-muted"
                on:click|stopPropagation={changeLikeStatus}
              >
                {#if asset.liked}
                  <Heart size={32} fill="red" color="red" class={animate ? "animate-ping" : ""} />
                {:else}
                  <Heart size={32} class={animate ? "animate-ping" : ""} />
                {/if}
              </button>
            </Tooltip.Trigger>
            <Tooltip.Content>
              <p>{asset.liked ? "Unlike" : "Like"} this Asset</p>
            </Tooltip.Content>
          </Tooltip.Root>
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
        <div class="flex items-center gap-2">
          <Heart size={18} fill="white" />
          {asset.likes}
          {asset.likes == 1 ? "Like" : "Likes"}
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
      {#if loading}
        <Button disabled variant="destructive">
          <Loader2 class="animate-spin" />
        </Button>
      {:else}
        <Button variant="destructive" on:click={deleteAsset(asset.id)}>Delete</Button>
      {/if}
    </div>
  </Dialog.Content>
</Dialog.Root>
