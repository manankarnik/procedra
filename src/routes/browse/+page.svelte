<script>
  import background from "$lib/assets/gradient.png";
  import * as Card from "$lib/components/ui/card";

  export let data;
</script>

<section>
  <div
    class="animate-gradient fixed top-0 z-[-10] w-[100vw] bg-gradient-to-r [mask-position:_top_center] [mask-repeat:_no-repeat] [mask-size:_cover] [mask-type:_alpha] sm:h-[1000px]"
    style="mask-image: url({background})"
  ></div>
  <div class="absolute top-0 z-[-10] h-[100vh] w-[100vw] backdrop-blur"></div>
  <div class="container p-8">
    <div>
      <h1
        class="animate-gradient bg-gradient-to-r bg-clip-text text-2xl font-extrabold text-transparent sm:text-3xl md:text-4xl lg:text-5xl"
      >
        Browse Assets
      </h1>
      <p class="text-muted-foreground">
        Immerse yourself in a world of infinite possibilities as you navigate through a vast
        collection of procedurally generated assets.
      </p>
    </div>
    <div
      class="my-4 columns-1 gap-4 sm:columns-2 md:columns-3 lg:columns-4"
    >
      {#each data.assets as asset}
        <button
          on:click={() => (window.location.href = `/generate/${asset.type}?id=${asset.id}`)}
          class="w-full my-2"
        >
          <Card.Root
            class="animate-gradient p-[2px] hover:border-transparent hover:bg-gradient-to-r"
          >
            <div class="flex flex-col rounded-md bg-background">
              <Card.Header class="flex justify-center">
                <img src={asset.thumbnail} alt="Asset thumbnail" />
              </Card.Header>
              <Card.Content>
                <Card.Title>{asset.title}</Card.Title>
                {#if asset.description}
                  <Card.Description>{asset.description}</Card.Description>
                {:else}
                  <Card.Description class="text-italic">No description provided</Card.Description>
                {/if}
              </Card.Content>
            </div>
          </Card.Root>
        </button>
      {/each}
    </div>
  </div>
</section>
