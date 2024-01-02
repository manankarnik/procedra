<script>
  import { signOut } from "@auth/sveltekit/client";
  import * as Popover from "$lib/components/ui/popover";
  import * as Avatar from "$lib/components/ui/avatar";
  import { Button } from "$lib/components/ui/button";
  import { User } from "lucide-svelte";
  export let session;
</script>

<Popover.Root>
  <Popover.Trigger>
    <Avatar.Root>
      <Avatar.Image src={session.user.image} alt="Profile" />
      <Avatar.Fallback>
        <User />
      </Avatar.Fallback>
    </Avatar.Root>
  </Popover.Trigger>
  <Popover.Content>
    <div class="flex items-center justify-center gap-4">
      <Avatar.Root>
        <Avatar.Image src={session.user.image} alt="Profile" />
        <Avatar.Fallback>
          <User />
        </Avatar.Fallback>
      </Avatar.Root>
      <strong>{session.user?.name ?? "User"}</strong>
    </div>
    <hr />
    <Button variant="destructive-outline" class="w-full" on:click={() => signOut()}>
      Sign out
    </Button>
  </Popover.Content>
</Popover.Root>
