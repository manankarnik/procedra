<script>
  import { Menu } from "lucide-svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Sheet from "$lib/components/ui/sheet";
  export let links;
</script>

<div class="sm:hidden">
  <Sheet.Root>
    <Sheet.Trigger>
      <Button variant="ghost" size="icon" aria-label="Menu">
        <Menu />
      </Button>
    </Sheet.Trigger>
    <Sheet.Content side="left" class="w-[200px]">
      <Sheet.Close class="text-left">
        {#each links as link}
          <a
            href={link.href}
            class="text-md block p-2 font-medium text-muted-foreground transition-colors hover:text-foreground"
          >
            {link.text}
          </a>
        {/each}
      </Sheet.Close>
    </Sheet.Content>
  </Sheet.Root>
</div>
