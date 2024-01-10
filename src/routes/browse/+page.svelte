<script>
  import background from "$lib/assets/gradient.png";
  import * as Card from "$lib/components/ui/card";
</script>

<section>
  <div
    class="animate-gradient absolute top-0 h-[600px] w-[100vw] bg-gradient-to-r [mask-position:_top_center] [mask-repeat:_no-repeat] [mask-size:_cover] [mask-type:_alpha] sm:h-[1000px]"
    style="mask-image: url({background})"
  ></div>
  <div class="z-[-10] w-full backdrop-blur md:h-[1000px]">
    <div class="container p-8 md:h-[800px]">
      <div class="py-2 md:py-0">
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
      <div class="my-4 grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3">
        {#each [1, 2, 3, 4, 5, 6, 7, 8] as i}
          <Card.Root
            class="animate-gradient h-full p-[2px] hover:border-transparent hover:bg-gradient-to-r"
          >
            <div class="flex h-full flex-col justify-between rounded-md bg-background">
              <Card.Header>
                <Card.Title>Card Title</Card.Title>
                <Card.Description>Card Description</Card.Description>
              </Card.Header>
              <Card.Content>
                <p>Card Content</p>
              </Card.Content>
              <Card.Footer>
                <p>Card Footer</p>
              </Card.Footer>
            </div>
          </Card.Root>
        {/each}
      </div>
    </div>
  </div>
</section>
