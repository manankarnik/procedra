<script context="module">
  import { get } from "svelte/store";
  import { page } from "$app/stores";
  export function recieve_asset() {
    return get(page).data.asset.data;
  }
</script>

<script>
  import { Loader2 } from "lucide-svelte";
  import { browser } from "$app/environment";
  import init_map from "../../../utils/map";
  import init_terrain from "../../../utils/terrain";
  import init_planet from "../../../utils/planet";

  export let util;
  const utils = { map: init_map, terrain: init_terrain, planet: init_planet };
  let init = utils[util];
</script>

<section class="container flex w-full items-center justify-center p-4">
  <div class="relative w-full">
    <canvas id="bevy-canvas" oncontextmenu="return false;" class="w-full" />
    {#if browser}
      {#await init()}
        <div
          class="absolute top-0 flex h-full w-full items-center justify-center bg-white dark:bg-black"
        >
          <Loader2 class="h-20 w-20 animate-spin" />
        </div>
      {/await}
    {/if}
  </div>
</section>
