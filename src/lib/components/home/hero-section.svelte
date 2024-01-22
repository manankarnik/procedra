<script>
  import { Button } from "$lib/components/ui/button";
  import terrain from "$lib/assets/terrain.gif";
  import background from "$lib/assets/gradient.png";
</script>

<section>
  <div
    class="animate-gradient absolute top-0 z-[-10] h-screen w-screen bg-gradient-to-r [mask-position:_top_center] [mask-repeat:_no-repeat] [mask-size:_cover] [mask-type:_alpha]"
    style="mask-image: url({background})"
  ></div>
  <div class="absolute top-0 z-[-10] h-[110vh] w-[110vw] backdrop-blur"></div>
  <div class="h-[80vh] container flex flex-col items-center justify-center gap-2">
    <div class="rounded-xl sm:p-10 sm:shadow-xl sm:backdrop-blur-xl">
      <h1
        class="animate-gradient scroll-m-20 bg-gradient-to-r bg-clip-text py-2 text-center text-2xl font-extrabold tracking-tight text-transparent sm:py-4 sm:text-4xl lg:text-5xl"
      >
        Procedural Marvels, Instantly.
      </h1>
      <p class="text-muted-foreground text-center text-center sm:text-lg">
        Procedural generation on the web in realtime!
      </p>
      <img class="mx-auto p-5 w-60 lg:w-80" src={terrain} />
      <div class="my-4 hidden items-center justify-center gap-4 sm:flex">
        <Button href="/browse" variant="outline">Browse Assets</Button>
        <Button href="/generate">Generate Assets</Button>
        <Button href="#more" variant="outline">Know More</Button>
      </div>
      <div class="my-4 flex flex-col items-center justify-center gap-2 sm:hidden">
        <Button href="/browse" size="sm" variant="outline" class="w-[300px]">Browse Assets</Button>
        <Button href="/generate" size="sm" class="w-[300px]">Generate Assets</Button>
        <Button href="#more" size="sm" variant="outline" class="w-[300px]">Know More</Button>
      </div>
    </div>
  </div>
</section>
