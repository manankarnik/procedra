<script>
  import { page } from "$app/stores";
  import DarkModeToggleButton from "$lib/components/dark-mode-toggle-button.svelte";
  import SignInDialog from "$lib/components/sign-in-dialog.svelte";
  import ProfilePopover from "$lib/components/profile-popover.svelte";
  import MobileMenu from "$lib/components/mobile-menu.svelte";

  let links = [
    { href: "/browse", text: "Browse" },
    { href: "/generate", text: "Generate" }
  ];
  if ($page.data.session) {
    links.push({ href: "/manage", text: "Manage" });
  }
  links.push({ href: "/dev", text: "For Devs" });
  links.push({ href: "https://github.com/manankarnik/procedra/releases", text: "Download" });
</script>

<header class="sticky top-0 z-10 border-b bg-[hsl(var(--background))]/60 backdrop-blur">
  <div class="container flex items-center justify-between px-8 py-4">
    <div class="flex items-center justify-between gap-2 sm:gap-0">
      <MobileMenu {links} />
      <a href="/" class="block flex items-center justify-center">
        <h3 class="px-2 py-0 text-xl font-bold sm:text-2xl">Procedra</h3>
      </a>
      {#each links as link}
        <a
          href={link.href}
          class="text-md mx-4 hidden font-medium text-muted-foreground transition-colors hover:text-foreground sm:block"
        >
          {link.text}
        </a>
      {/each}
    </div>
    <div class="flex items-center justify-center gap-4">
      {#if $page.data.session}
        <ProfilePopover session={$page.data.session} />
      {:else}
        <SignInDialog />
      {/if}
      <DarkModeToggleButton />
    </div>
  </div>
</header>
