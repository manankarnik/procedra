<script>
  import "../app.pcss";
  import { ModeWatcher } from "mode-watcher";
  import Header from "$lib/components/header.svelte";
</script>

<ModeWatcher />
<Header />
<slot />
