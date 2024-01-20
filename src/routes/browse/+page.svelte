<script>
  import background from "$lib/assets/gradient.png";
  import AssetCard from "$lib/components/asset-card.svelte";

  export let data;
</script>

<section class="container p-8">
  <div>
    <h1
      class="animate-gradient bg-gradient-to-r bg-clip-text text-2xl font-extrabold text-transparent sm:text-3xl md:text-4xl lg:text-5xl"
    >
      Browse Assets
    </h1>
    <p class="text-muted-foreground">
      Immerse yourself in a world of infinite possibilities as you navigate through a vast
      collection of procedurally generated assets.
    </p>
  </div>
  <div class="grid-col-1 my-4 grid gap-8 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
    {#each data.assets as asset}
      <AssetCard {asset} />
    {/each}
  </div>
</section>
