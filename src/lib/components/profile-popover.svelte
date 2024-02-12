<script>
  import { signOut } from "@auth/sveltekit/client";
  import { browser } from "$app/environment";
  import { page } from "$app/stores";
  import * as Popover from "$lib/components/ui/popover";
  import * as Avatar from "$lib/components/ui/avatar";
  import * as Select from "$lib/components/ui/select";
  import { Button } from "$lib/components/ui/button";
  import { User } from "lucide-svelte";
  import { mode } from "mode-watcher";
  import colors from "tailwindcss/colors";

  export let session;

  const tiers = ["primary", "secondary"];
  const theme = [$page.data.primary, $page.data.secondary].map((color) => {
    return { value: color, label: color.charAt(0).toUpperCase() + color.slice(1), disabled: false };
  });
  const tailwindColors = [
    "red",
    "orange",
    "amber",
    "yellow",
    "lime",
    "green",
    "emerald",
    "teal",
    "cyan",
    "sky",
    "blue",
    "indigo",
    "violet",
    "purple",
    "fuchsia",
    "pink",
    "rose"
  ];

  $: if (browser) {
    document
      .querySelector(":root")
      .style.setProperty("--primary-color", colors[theme[0].value][$mode == "dark" ? 600 : 500]);
    document
      .querySelector(":root")
      .style.setProperty("--secondary-color", colors[theme[1].value][$mode == "dark" ? 600 : 500]);
  }
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
  <Popover.Content class="w-50">
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
    {#each tiers as tier}
      <div class="py-2">
        <h4 class="text-sm font-semibold">{tier.charAt(0).toUpperCase() + tier.slice(1)}</h4>
        <div class="my-2 flex items-center justify-center gap-4">
          <div class={"h-8 w-8 rounded-full" + ` bg-${theme[tiers.indexOf(tier)].value}-500`}></div>
          <Select.Root
            onSelectedChange={(color) => {
              fetch("/theme", {
                method: "POST",
                body: JSON.stringify({ tier, color: color.value, userId: $page.data.userId })
              });
            }}
            bind:selected={theme[tiers.indexOf(tier)]}
            preventScroll={false}
          >
            <Select.Trigger class="w-[180px]">
              <Select.Value />
            </Select.Trigger>
            <Select.Content>
              {#each tailwindColors as color}
                <Select.Item value={color}
                  >{color.charAt(0).toUpperCase() + color.slice(1)}</Select.Item
                >
              {/each}
            </Select.Content>
          </Select.Root>
        </div>
      </div>
    {/each}
    <hr />
    <Button variant="destructive-outline" class="w-full" on:click={() => signOut()}>
      Sign out
    </Button>
  </Popover.Content>
</Popover.Root>
