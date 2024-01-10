<script>
  import { Button } from "$lib/components/ui/button";
  import background from "$lib/assets/gradient.png";
</script>

<section>
  <div
    class="animate-gradient absolute top-0 h-[600px] w-[100vw] bg-gradient-to-r [mask-position:_top_center] [mask-repeat:_no-repeat] [mask-size:_cover] [mask-type:_alpha] sm:h-[1000px]"
    style="mask-image: url({background})"
  ></div>
  <div class="h-[600px] w-full backdrop-blur sm:h-[1000px]">
    <div
      class="container z-[10] flex h-[500px] flex-col items-center justify-center gap-2 sm:h-[700px]"
    >
      <h1
        class="animate-gradient scroll-m-20 bg-gradient-to-r bg-clip-text py-2 text-center text-2xl font-extrabold tracking-tight text-transparent sm:py-4 sm:text-4xl lg:text-5xl"
      >
        Procedural Marvels, Instantly.
      </h1>
      <p class="text-accent-muted-fg text-center text-center sm:text-lg">
        Procedural generation on the web in realtime!
      </p>
      <div class="hidden sm:flex my-4 justify-center items-center gap-4">
        <Button href="/browse" variant="outline">View Assets</Button>
        <Button href="generate">Generate Assets</Button>
      </div>
      <div class="flex sm:hidden flex-col my-4 justify-center items-center gap-2">
        <Button href="/browse" size="sm" variant="outline" class="w-[300px]">View Assets</Button>
        <Button href="generate" size="sm" class="w-[300px]">Generate Assets</Button>
      </div>
    </div>
  </div>
</section>
