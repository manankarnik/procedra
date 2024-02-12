<script>
  import { page } from "$app/stores";
  import { Loader2, CheckCircle2 } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import * as Alert from "$lib/components/ui/alert";
  import AssetCard from "$lib/components/asset-card.svelte";
  import SearchFilter from "$lib/components/search-filter.svelte";
  import Footer from "$lib/components/footer.svelte";

  export let data;
  let filteredAssets = data.assets;
  let open = false;
  let loading;
  let alert;
  let currentAsset;

  function deleteDialog(id) {
    open = true;
    currentAsset = data.assets.find(asset => asset.id == id);
  }
  async function deleteAsset(id) {
    loading = true;
    await fetch("/asset", { method: "DELETE", body: JSON.stringify({ id }) });
    loading = false;
    open = false;
    data.assets = data.assets.filter(asset => asset.id != id);
    alert = true;
    setTimeout(() => {
      alert = false;
    }, 2000);
  }
</script>

<div class={`fixed left-0 top-20 flex h-20 w-full justify-center ${alert ? "" : "hidden"}`}>
  <Alert.Root variant="success" class="w-30 bg-slate-100 dark:bg-slate-900">
    <CheckCircle2 class="h-4 w-4 text-green-500" />
    <Alert.Title>Success</Alert.Title>
    <Alert.Description>Asset deleted successfully!</Alert.Description>
  </Alert.Root>
</div>
<section class="container p-8">
  <h1
    class="animate-gradient bg-gradient-to-r bg-clip-text text-2xl font-extrabold text-transparent sm:text-3xl md:text-4xl lg:pb-1 lg:text-5xl"
  >
    Manage Assets
  </h1>
  <p class="mb-2 text-muted-foreground">Edit or delete your assets</p>
  <SearchFilter assets={data.assets} bind:filteredAssets />
  {#if filteredAssets.length == 0}
    <div class="m-10 flex justify-center text-xl sm:text-2xl">No Assets found :(</div>
  {:else}
    <div class="grid-col-1 my-4 mt-2 grid gap-8 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4">
      {#each filteredAssets as asset}
        <AssetCard {asset} manage={true} {deleteDialog} />
      {/each}
    </div>
  {/if}
</section>
<Dialog.Root bind:open>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Delete Asset</Dialog.Title>
      <Dialog.Description>
        Are you sure you want to <span class="font-bold text-destructive">DELETE</span> the asset
        <span class="font-bold text-primary-color">{currentAsset.title}</span>?
      </Dialog.Description>
    </Dialog.Header>
    <div class="flex justify-end gap-4">
      <Button variant="outline" on:click={() => (open = false)}>Cancel</Button>
      {#if loading}
        <Button disabled variant="destructive">
          <Loader2 class="animate-spin" />
        </Button>
      {:else}
        <Button variant="destructive" on:click={deleteAsset(currentAsset.id)}>Delete</Button>
      {/if}
    </div>
  </Dialog.Content>
</Dialog.Root>
<Footer />
