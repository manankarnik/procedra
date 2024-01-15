<script context="module">
  import { get } from "svelte/store";
  import { page } from "$app/stores";
  export function recieve_asset() {
    return get(page).data.asset;
  }
</script>

<script>
  import Util from "$lib/components/generate/util.svelte";
  import PublishPopup from "$lib/components/generate/publish-popup.svelte";
  import background from "$lib/assets/gradient.png";
  import init_map from "../../../utils/map";
  import init_terrain from "../../../utils/terrain";
  import init_planet from "../../../utils/planet";

  /** @type {import('./$types').PageData} */
  export let data;
  const utils = { map: init_map, terrain: init_terrain, planet: init_planet };
  let init = utils[data.util];
</script>

<section>
  <Util {init} />
  <PublishPopup util={data.util} session={$page.data.session} />
</section>
