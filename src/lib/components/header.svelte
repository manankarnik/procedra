<script>
  import { page } from "$app/stores";
  import DarkModeToggleButton from "$lib/components/dark-mode-toggle-button.svelte";
  import SignInDialog from "$lib/components/sign-in-dialog.svelte";
  import ProfilePopover from "$lib/components/profile-popover.svelte";

  const links = [
    { href: "#", text: "Browse" },
    { href: "/generate", text: "Generate" },
    { href: "#", text: "Blog" }
  ];
</script>

<div class="sticky top-0 z-10 border-b bg-[hsl(var(--background))]/60 backdrop-blur">
  <div class="container flex items-center justify-between p-4">
    <div class="flex items-center justify-between gap-4">
      <a href="/">
        <h3 class="px-2 text-lg font-bold">Procedra</h3>
      </a>
      {#each links as link}
        <a
          href={link.href}
          class="text-md font-medium text-muted-foreground transition-colors hover:text-foreground"
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
</div>
