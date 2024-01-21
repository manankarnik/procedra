<script>
  import { page } from "$app/stores";
  import { User, Heart, MoreVertical, Pencil, Trash } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import * as Avatar from "$lib/components/ui/avatar";
  import * as Tooltip from "$lib/components/ui/tooltip";

  export let asset;
  export let manage = false;
  export let deleteDialog = () => {};
  let animate;
  async function changeLikeStatus() {
    animate = true;
    setTimeout(() => {
      animate = false;
    }, 250);
    asset.liked = !asset.liked;
    asset.likes = asset.liked ? asset.likes + 1 : asset.likes - 1;
    await fetch("/like", {
      method: "POST",
      body: JSON.stringify({
        assetId: asset.id,
        userId: $page.data.userId
      })
    });
  }
</script>

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
                on:click={() => deleteDialog(asset.id)}
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
