<script>
  import Util from "$lib/components/generate/util.svelte";
  import PublishPopup from "$lib/components/generate/publish-popup.svelte";
  import { page } from "$app/stores";

  /** @type {import('./$types').PageData} */
  export let data;
</script>

<svelte:head>
  <title>{data.util.charAt(0).toUpperCase() + data.util.slice(1)} • Procedra</title>
</svelte:head>

<section>
  <Util util={data.util} />
  <PublishPopup util={data.util} session={$page.data.session} />
</section>
