<script context="module">
  import { writable, get } from "svelte/store";
  import { mode } from "mode-watcher";

  const publishPopup = writable(false);
  let asset;
  let thumbnail;
  export function send_asset(asset_, thumbnail_) {
    asset = asset_;
    thumbnail = Array.from(thumbnail_);
    publishPopup.set(true);
  }
  export function dark_theme() {
    return get(mode) == "dark";
  }
</script>

<script>
  import * as Dialog from "$lib/components/ui/dialog";
  import * as Select from "$lib/components/ui/select";
  import { Label } from "$lib/components/ui/label";
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import SignInDialog from "$lib/components/sign-in-dialog.svelte";

  export let util;
  export let session;

  let open = false;
  $: open = $publishPopup;

  let title;
  let description;
  let visibility = { value: "public", label: "Public", disabled: false };

  async function publish() {
    await fetch(`/publish`, {
      method: "POST",
      body: JSON.stringify({
        title,
        description,
        public: visibility.value == "public",
        type: util,
        data: asset,
        thumbnail
      })
    });
    publishPopup.set(false);
    title = "";
    description = "";
    visibility = { value: "public", label: "Public", disabled: false };
  }
</script>

{#if session}
  <Dialog.Root {open} onOpenChange={(value) => publishPopup.set(value)} closeOnOutsideClick={false}>
    <Dialog.Content>
      <Dialog.Header>
        <Dialog.Title>Publish Asset</Dialog.Title>
        <Dialog.Description>We need a few more details before you can publish</Dialog.Description>
      </Dialog.Header>
      <form on:submit={publish} class="flex flex-col gap-4">
        <div>
          <Label for="title">
            Title<span class="text-red-500">*</span>
          </Label>
          <Input name="title" required bind:value={title} class="mt-2" />
        </div>
        <div>
          <Label for="title">Visibilty</Label>
          <Select.Root bind:selected={visibility}>
            <Select.Trigger>
              <Select.Value />
            </Select.Trigger>
            <Select.Content>
              <Select.Item value="public">Public</Select.Item>
              <Select.Item value="private">Private</Select.Item>
            </Select.Content>
          </Select.Root>
        </div>
        <div>
          <Label for="description">Description</Label>
          <Input name="description" bind:value={description} class="mt-2" />
        </div>
        <Button type="submit">Publish</Button>
      </form>
    </Dialog.Content>
  </Dialog.Root>
{:else}
  <SignInDialog {open} showTrigger={false} />
{/if}
