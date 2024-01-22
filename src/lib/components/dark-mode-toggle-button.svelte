<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import { Sun, Moon } from "lucide-svelte";
  import { setMode, resetMode } from "mode-watcher";
</script>

<DropdownMenu.Root preventScroll={false}>
  <DropdownMenu.Trigger>
    <Button variant="ghost" size="icon">
      <Sun class="h-[1.5rem] w-[1.5rem] dark:hidden" />
      <Moon class="hidden h-[1.5rem] w-[1.5rem] dark:block" />
      <span class="sr-only">Toggle theme</span>
    </Button>
  </DropdownMenu.Trigger>
  <DropdownMenu.Content>
    <DropdownMenu.Item on:click={() => setMode("light")}>Light</DropdownMenu.Item>
    <DropdownMenu.Item on:click={() => setMode("dark")}>Dark</DropdownMenu.Item>
    <DropdownMenu.Item on:click={() => resetMode()}>System</DropdownMenu.Item>
  </DropdownMenu.Content>
</DropdownMenu.Root>
