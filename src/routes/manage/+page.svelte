<script>
  import background from "$lib/assets/gradient.png";
  import AssetCard from "$lib/components/asset-card.svelte";

  export let data;
</script>

<section class="container p-8">
  <h1
    class="animate-gradient bg-gradient-to-r bg-clip-text text-2xl font-extrabold text-transparent sm:text-3xl md:text-4xl lg:pb-1 lg:text-5xl"
  >
    Manage Assets
  </h1>
  <div class="grid-col-1 my-4 grid gap-8 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
    {#each data.assets as asset}
      <AssetCard {asset} manage={true} />
    {/each}
  </div>
</section>
