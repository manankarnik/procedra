<script>
  import DarkModeToggleButton from "$lib/components/dark-mode-toggle-button.svelte";
  import { Button } from "$lib/components/ui/button";
</script>

<div class="sticky top-0 border border-b backdrop-blur">
  <div class="container flex items-center justify-between p-4">
    <div class="flex items-center justify-between gap-4">
      <h3 class="px-2 text-lg font-bold">Procedra</h3>
      <a
        href="#"
        class="text-md font-medium text-muted-foreground transition-colors hover:text-primary"
      >
        Browse
      </a>
      <a
        href="#"
        class="text-md font-medium text-muted-foreground transition-colors hover:text-primary"
      >
        Generate
      </a>
      <a
        href="#"
        class="text-md font-medium text-muted-foreground transition-colors hover:text-primary"
      >
        Blog
      </a>
    </div>
    <div class="flex items-center justify-between gap-4">
      <Button variant="primary">Sign Up</Button>
      <DarkModeToggleButton />
    </div>
  </div>
</div>
