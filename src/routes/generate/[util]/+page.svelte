<script>
  import { page } from "$app/stores";
  import Util from "$lib/components/generate/util.svelte";
  import PublishPopup from "$lib/components/generate/publish-popup.svelte";
  import background from "$lib/assets/gradient.png";

  /** @type {import('./$types').PageData} */
  export let data;
</script>

<section>
  <div
    class="animate-gradient absolute top-0 z-[-10] h-[1000px] w-full bg-gradient-to-r [mask-position:_top_center] [mask-repeat:_no-repeat] [mask-size:_cover] [mask-type:_alpha]"
    style="mask-image: url({background})"
  ></div>
  <div class="z-[-10] h-[1000px] w-full backdrop-blur">
    <Util init={data.init} />
  </div>
  <PublishPopup util={data.util} session={$page.data.session} />
</section>
