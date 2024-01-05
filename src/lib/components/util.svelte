<script>
  import init_map from "../../utils/map";
  import init_terrain from "../../utils/terrain";
  import init_planet from "../../utils/planet";
  import { Loader2 } from "lucide-svelte";

  export let util;
  const utils = { map: init_map, terrain: init_terrain, planet: init_planet };
</script>

<section class="flex w-full items-center justify-center p-4">
  <div class="relative h-[720px] w-[1280px]">
    {#await utils[util]()}
      <div class="absolute top-0 z-[-10] flex h-full w-full items-center justify-center bg-black">
        <Loader2 class="h-20 w-20 animate-spin" />
      </div>
    {/await}
    <canvas id="bevy-canvas" width="1280" height="720" oncontextmenu="return false;" />
  </div>
</section>
