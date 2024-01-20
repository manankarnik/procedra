<script>
  import background from "$lib/assets/gradient.png";
  import AssetCard from "$lib/components/asset-card.svelte";
  import SearchFilter from "$lib/components/search-filter.svelte";

  export let data;
  let filteredAssets = data.assets;
</script>

<section class="container p-8">
  <h1
    class="animate-gradient bg-gradient-to-r bg-clip-text text-2xl font-extrabold text-transparent sm:text-3xl md:text-4xl lg:pb-1 lg:text-5xl"
  >
    Manage Assets
  </h1>
  <p class="mb-2 text-muted-foreground">Edit or delete your assets</p>
  <SearchFilter assets={data.assets} bind:filteredAssets filter={false} />
  {#if filteredAssets.length == 0}
    <div class="m-10 flex justify-center text-xl sm:text-2xl">No Assets found :(</div>
  {:else}
    <div class="grid-col-1 my-4 mt-2 grid gap-8 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
      {#each filteredAssets as asset}
        <AssetCard {asset} manage={true} />
      {/each}
    </div>
  {/if}
</section>
