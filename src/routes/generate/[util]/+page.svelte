<script>
  import Util from "$lib/components/util.svelte";

  /** @type {import('./$types').PageData} */
  export let data;
</script>

<Util init={data.init} />
