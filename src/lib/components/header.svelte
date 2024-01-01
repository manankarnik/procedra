<script>
  import DarkModeToggleButton from "$lib/components/dark-mode-toggle-button.svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
</script>

<div class="sticky top-0 z-10 border-b bg-[hsl(var(--background))]/60 backdrop-blur">
  <div class="container flex items-center justify-between p-4">
    <div class="flex items-center justify-between gap-4">
      <h3 class="px-2 text-lg font-bold">Procedra</h3>
      <a
        href="#"
        class="text-md font-medium text-muted-foreground transition-colors hover:text-foreground"
      >
        Browse
      </a>
      <a
        href="#"
        class="text-md font-medium text-muted-foreground transition-colors hover:text-foreground"
      >
        Generate
      </a>
      <a
        href="#"
        class="text-md font-medium text-muted-foreground transition-colors hover:text-foreground"
      >
        Blog
      </a>
    </div>
    <div class="flex items-center justify-between gap-4">
      <Dialog.Root>
        <Dialog.Trigger>
          <Button variant="primary">Sign In</Button>
        </Dialog.Trigger>
        <Dialog.Content>
          <Dialog.Header>
            <Dialog.Title>Sign In</Dialog.Title>
            <Dialog.Description>
              Sign In to <span class="font-bold">Procedra</span> using one of the following providers
            </Dialog.Description>
          </Dialog.Header>
          <Button variant="outline" size="lg" class="flex items-center justify-center gap-3">
            <img
              src="https://www.svgrepo.com/show/355037/google.svg"
              class="h-5 w-5"
              alt="Google Logo"
            />
            <span>Continue with Google</span>
          </Button>
          <Button variant="outline" size="lg" class="flex items-center justify-center gap-3">
            <img
              src="https://www.svgrepo.com/show/512317/github-142.svg"
              class="h-5 w-5 dark:invert"
              alt="Github Logo"
            />
            <span>Continue with Github</span>
          </Button>
          <Button variant="outline" size="lg" class="flex items-center justify-center gap-3">
            <img
              src="https://assets-global.website-files.com/6257adef93867e50d84d30e2/636e0a69f118df70ad7828d4_icon_clyde_blurple_RGB.svg"
              class="h-5 w-5"
              alt="Discord Logo"
            />
            <span>Continue with Discord</span>
          </Button>
        </Dialog.Content>
      </Dialog.Root>
      <DarkModeToggleButton />
    </div>
  </div>
</div>
