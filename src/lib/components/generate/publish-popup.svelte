<script context="module">
  import { writable, get } from "svelte/store";
  import { mode } from "mode-watcher";

  const publishPopup = writable(false);
  let assetData;
  let thumbnail;
  export function send_asset(assetData_, thumbnail_) {
    assetData = assetData_;
    thumbnail = Array.from(thumbnail_);
    publishPopup.set(true);
  }
  export function dark_theme() {
    return get(mode) == "dark";
  }
</script>

<script>
  import { enhance } from "$app/forms";
  import { page } from "$app/stores";
  import * as Dialog from "$lib/components/ui/dialog";
  import * as Select from "$lib/components/ui/select";
  import { Label } from "$lib/components/ui/label";
  import { Input } from "$lib/components/ui/input";
  import { Button } from "$lib/components/ui/button";
  import SignInDialog from "$lib/components/sign-in-dialog.svelte";

  export let util;
  export let session;
  export let asset;

  let open = false;
  $: open = $publishPopup;

  let visibilities = [
    { value: "public", label: "Public", disabled: false },
    { value: "private", label: "private", disabled: false }
  ];
  let title = asset ? asset.title : null;
  let description = asset ? asset.description : null;
  let visibility = asset
    ? visibilities.find((visibility) => {
        if (asset.public) {
          return visibility.value == "public";
        } else {
          return visibility.value == "private";
        }
      })
    : visibilities[0];

  async function publish(type) {
    const data = {
      title,
      description,
      public: visibility.value == "public",
      type: util,
      data: assetData,
      thumbnail
    };
    if (type == "create") {
      data.userId = $page.data.userId;
    } else {
      data.id = asset.id;
    }
    await fetch(`/asset`, {
      method: `${type == "create" ? "POST" : "PUT"}`,
      body: JSON.stringify(data)
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
        {#if asset}
          <Dialog.Title>Update Asset</Dialog.Title>
          <Dialog.Description>Review and update your asset details</Dialog.Description>
        {:else}
          <Dialog.Title>Publish Asset</Dialog.Title>
          <Dialog.Description>We need a few more details before you can publish</Dialog.Description>
        {/if}
      </Dialog.Header>
      <form
        class="flex flex-col gap-4"
        on:submit={() => (asset ? publish("update") : publish("create"))}
      >
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
