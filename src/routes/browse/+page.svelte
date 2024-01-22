<script>
  import AssetCard from "$lib/components/asset-card.svelte";
  import SearchFilter from "$lib/components/search-filter.svelte";
  import Footer from "$lib/components/footer.svelte";
  import { page } from "$app/stores";

  export let data;
  let filteredAssets = data.assets;
</script>

<section class="container p-8">
  <div>
    <h1
      class="animate-gradient bg-gradient-to-r bg-clip-text text-2xl font-extrabold text-transparent sm:text-3xl md:text-4xl lg:text-5xl"
    >
      Browse Assets
    </h1>
    <p class="mb-2 text-muted-foreground">
      Immerse yourself in a world of infinite possibilities as you navigate through a vast
      collection of procedurally generated assets.
    </p>
  </div>
  <SearchFilter assets={data.assets} bind:filteredAssets filter={Boolean($page.data.session)} />
  {#if filteredAssets.length == 0}
    <div class="m-10 flex justify-center text-xl sm:text-2xl">No Assets found :(</div>
  {:else}
    <div class="grid-col-1 my-4 mt-2 grid gap-8 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
      {#each filteredAssets as asset}
        <AssetCard {asset} />
      {/each}
    </div>
  {/if}
</section>
<Footer />
