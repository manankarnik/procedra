<script>
  import { goto } from "$app/navigation";
  import * as Card from "$lib/components/ui/card";
  import map from "$lib/assets/map.png";
  import terrain from "$lib/assets/terrain.png";
  import planet from "$lib/assets/planet.png";

  const utils = [
    {
      href: "map",
      title: "Textures/Maps",
      description: "Facilitates generation of 2D textures and maps",
      logo: map
    },
    {
      href: "terrain",
      title: "Terrain",
      description: "Facilitates generation of 3D terrain",
      logo: terrain
    },
    {
      href: "planet",
      title: "Planets",
      description: "Facilitates generation of planets and asteroids",
      logo: planet
    }
  ];
</script>

<section class="container flex h-[800px] items-center justify-center p-4">
  <div class="flex flex-col gap-6">
    <h1
      class="animate-gradient bg-gradient-to-r bg-clip-text text-4xl font-extrabold text-transparent lg:text-5xl"
    >
      What would you like to create?
    </h1>
    <div class="grid grid-cols-3 content-stretch gap-4">
      {#each utils as util}
        <button on:click={() => (window.location.href = `generate/${util.href}`)}>
          <Card.Root
            class="animate-gradient h-full p-[1px] hover:border-transparent hover:bg-gradient-to-r"
          >
            <div class="flex h-full flex-col justify-between rounded-md bg-background">
              <Card.Header>
                <Card.Title>{util.title}</Card.Title>
                <Card.Description>{util.description}</Card.Description>
              </Card.Header>
              <Card.Content class="flex items-center justify-center">
                <div
                  class="animate-gradient h-40 w-full bg-gradient-to-r bg-contain [mask-position:_top_center] [mask-repeat:_no-repeat] [mask-size:_contain] [mask-type:_alpha]"
                  style="mask-image: url({util.logo})"
                ></div>
              </Card.Content>
            </div>
          </Card.Root>
        </button>
      {/each}
    </div>
  </div>
</section>

<style>
</style>
