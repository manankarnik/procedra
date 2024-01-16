<script>
  import { User, Heart } from "lucide-svelte";
  export let asset;
</script>

<div class="w-full rounded-lg">
  <button
    class="group relative w-full"
    on:click={() => (window.location.href = `/generate/${asset.type}?id=${asset.id}`)}
  >
    <img
      class="p-4 flex h-[200px] bg-slate-100 dark:bg-slate-900 w-full items-center justify-center rounded-lg object-contain"
      src={asset.thumbnail}
      alt="Asset thumbnail"
    />
    <div
      class="absolute bottom-0 w-full bg-background/60 p-4 text-start opacity-0 backdrop-blur transition-opacity ease-in-out group-hover:opacity-100"
    >
      <h2 class="text-lg md:text-xl">{asset.title}</h2>
      {#if asset.description}
        <p>{asset.description}</p>
      {:else}
        <p class="font-italic text-muted-foreground">No description provided</p>
      {/if}
    </div>
  </button>
  <div class="font-extrabold flex justify-between p-2">
    <div class="flex gap-2">
      <User />
      User
    </div>
    <div class="flex gap-2">
      <Heart />
      1K
    </div>
  </div>
</div>
