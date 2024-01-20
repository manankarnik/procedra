<script>
  import "../app.pcss";
  import { ModeWatcher } from "mode-watcher";
  import Header from "$lib/components/header.svelte";
  import LoadingBar from "$lib/components/loading-bar.svelte";
</script>

<LoadingBar />
<ModeWatcher />
<Header />
<slot />
