<script>
  import { Loader2 } from "lucide-svelte";
  export let init;
</script>

<section class="container flex w-full items-center justify-center p-4">
  <div class="relative w-full">
    <canvas id="bevy-canvas" oncontextmenu="return false;" class="w-full" />
    {#await init()}
      <div class="absolute top-0 flex h-full w-full items-center justify-center bg-black">
        <Loader2 class="h-20 w-20 animate-spin" />
      </div>
    {/await}
  </div>
</section>
