<script>
  import "../app.pcss";
  import { ModeWatcher } from "mode-watcher";
  import Header from "$lib/components/header.svelte";
</script>

<div
  class="y-0 absolute z-[-10] h-full w-full bg-[url('$lib/assets/gradient.png')] bg-cover bg-no-repeat"
></div>
<ModeWatcher />
<Header />
<div class="backdrop-blur-xl">
  <slot class="container" />
</div>
