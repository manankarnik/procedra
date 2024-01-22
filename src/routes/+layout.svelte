<script>
  import "../app.pcss";
  import { ModeWatcher } from "mode-watcher";
  import Header from "$lib/components/header.svelte";
  import LoadingBar from "$lib/components/loading-bar.svelte";
  import BackgroundGradient from "$lib/components/background-gradient.svelte";
  import GithubRibbon from "$lib/components/github-ribbon.svelte";
</script>

<BackgroundGradient />
<LoadingBar />
<ModeWatcher />
<Header />
<GithubRibbon />
<slot />
