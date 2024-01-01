<script>
  import "../app.pcss";
  import { ModeWatcher } from "mode-watcher";
  import DarkModeToggleButton from "$lib/components/dark-mode-toggle-button.svelte";
</script>

<ModeWatcher />
<DarkModeToggleButton />
<slot />
