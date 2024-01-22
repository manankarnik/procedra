<script>
  import { page } from "$app/stores";
  import { Input } from "$lib/components/ui/input";
  import { Search } from "lucide-svelte";
  import * as Select from "$lib/components/ui/select";

  export let assets;
  export let filteredAssets;
  export let filter = false;
  let search = "";
  let typesList = [
    { value: "map", label: "Maps/Textures", disabled: false },
    { value: "terrain", label: "Terrain", disabled: false },
    { value: "planet", label: "Planets", disabled: false }
  ];
  let filtersList = [
    { value: "liked", label: "Liked by me", disabled: false },
    { value: "created", label: "Created by me", disabled: false }
  ];
  let types = [];
  let filters = [];
  $: {
    filteredAssets = assets.filter(
      (asset) =>
        asset.title.toLowerCase().includes(search.toLowerCase()) ||
        asset.description?.toLowerCase().includes(search.toLowerCase()) ||
        asset.user.name.toLowerCase().includes(search.toLowerCase())
    );
    filteredAssets = filteredAssets.filter(
      (asset) => types.map((t) => t.value).includes(asset.type) || types.length == 0
    );
    if (filter) {
      filteredAssets = filteredAssets.filter((asset) => {
        if (filters.map((f) => f.value).includes("liked")) {
          return asset.liked;
        }
        if (filters.map((f) => f.value).includes("created")) {
          return asset.user.id == $page.data.userId;
        }
        return true;
      });
    }
  }
</script>

<div class="flex flex-col items-center justify-center gap-0 md:flex-row md:gap-4">
  <Input bind:value={search} placeholder="Search by title, description or user" class="my-2 gap-2">
    <Search slot="prepend" size={22} class="text-muted-foreground" />
  </Input>
  <Select.Root multiple bind:selected={types}>
    <Select.Trigger class="my-2 p-2">
      <div class="inline truncate text-nowrap">
        <span class="text-md rounded-md bg-accent p-1">Type</span>
        <Select.Value placeholder="All" />
      </div>
    </Select.Trigger>
    <Select.Content>
      {#each typesList as type}
        <Select.Item value={type.value}>{type.label}</Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>
  {#if filter}
    <Select.Root multiple bind:selected={filters}>
      <Select.Trigger class="my-2 p-2">
        <div class="inline truncate text-nowrap">
          <span class="text-md rounded-md bg-accent p-1">Filter</span>
          <Select.Value placeholder="All" />
        </div>
      </Select.Trigger>
      <Select.Content>
        {#each filtersList as filter}
          <Select.Item value={filter.value}>{filter.label}</Select.Item>
        {/each}
      </Select.Content>
    </Select.Root>
  {/if}
</div>
