<script>
  import { Loader2 } from "lucide-svelte";
  export let init;
</script>

<section class="flex w-full items-center justify-center p-4">
  <div class="relative h-[720px] w-[1280px]">
    <canvas id="bevy-canvas" width="1280" height="720" oncontextmenu="return false;" />
    {#await init()}
      <div class="absolute top-0 flex h-full w-full items-center justify-center bg-black">
        <Loader2 class="h-20 w-20 animate-spin" />
      </div>
    {/await}
  </div>
</section>
