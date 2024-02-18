<script>
  import Util from "$lib/components/generate/util.svelte";
  import PublishPopup from "$lib/components/generate/publish-popup.svelte";
  import { page } from "$app/stores";

  /** @type {import('./$types').PageData} */
  export let data;
</script>

<svelte:head>
  <title>{data.asset.title} • Procedra</title>
</svelte:head>

<section>
  <Util util={data.util} />
  <PublishPopup util={data.util} session={$page.data.session} asset={data.asset} />
</section>
