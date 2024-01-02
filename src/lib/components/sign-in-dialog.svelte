<script>
  import { signIn } from "@auth/sveltekit/client";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Button } from "$lib/components/ui/button";
</script>

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
    <Button
      on:click={() => signIn("google")}
      variant="outline"
      size="lg"
      class="flex items-center justify-center gap-3"
    >
      <img src="https://www.svgrepo.com/show/355037/google.svg" class="h-5 w-5" alt="Google Logo" />
      <span>Continue with Google</span>
    </Button>
    <Button
      on:click={() => signIn("github")}
      variant="outline"
      size="lg"
      class="flex items-center justify-center gap-3"
    >
      <img
        src="https://www.svgrepo.com/show/512317/github-142.svg"
        class="h-5 w-5 dark:invert"
        alt="Github Logo"
      />
      <span>Continue with Github</span>
    </Button>
    <Button
      on:click={() => signIn("discord")}
      variant="outline"
      size="lg"
      class="flex items-center justify-center gap-3"
    >
      <img
        src="https://assets-global.website-files.com/6257adef93867e50d84d30e2/636e0a69f118df70ad7828d4_icon_clyde_blurple_RGB.svg"
        class="h-5 w-5"
        alt="Discord Logo"
      />
      <span>Continue with Discord</span>
    </Button>
  </Dialog.Content>
</Dialog.Root>
