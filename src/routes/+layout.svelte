<script>
  import "../app.pcss";
  import { ModeWatcher } from "mode-watcher";
  import Header from "$lib/components/header.svelte";
  import LoadingBar from "$lib/components/loading-bar.svelte";
  import BackgroundGradient from "$lib/components/background-gradient.svelte";
  import GithubRibbon from "$lib/components/github-ribbon.svelte";
  import { page } from "$app/stores";
  import { browser } from "$app/environment";
  import { mode } from "mode-watcher";
  import colors from "tailwindcss/colors";

  if (browser) {
    document
      .querySelector(":root")
      .style.setProperty(
        "--primary-color",
        colors[$page.data.primary][$mode == "dark" ? 600 : 500]
      );
    document
      .querySelector(":root")
      .style.setProperty(
        "--secondary-color",
        colors[$page.data.secondary][$mode == "dark" ? 600 : 500]
      );
  }
</script>

<BackgroundGradient />
<LoadingBar />
<ModeWatcher />
<Header />
<GithubRibbon />
<slot />
