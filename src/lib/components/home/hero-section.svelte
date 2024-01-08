<script>
  import { Button } from "$lib/components/ui/button";
  import background from "$lib/assets/gradient.png";
</script>

<section>
  <div
    class="animate-gradient absolute top-0 h-[1000px] w-full bg-gradient-to-r [mask-position:_top_center] [mask-repeat:_no-repeat] [mask-size:_cover] [mask-type:_alpha]"
    style="mask-image: url({background})"
  ></div>
  <div class="h-[1000px] w-full backdrop-blur">
    <div class="container z-[10] flex h-[700px] flex-col items-center justify-center gap-2">
      <h1
        class="animate-gradient scroll-m-20 bg-gradient-to-r bg-clip-text py-4 text-center text-4xl font-extrabold tracking-tight text-transparent lg:text-5xl"
      >
        Procedural Marvels, Instantly.
      </h1>
      <p class="text-accent-muted-fg text-center text-center text-lg">
        Procedural generation on the web in realtime!
      </p>
      <div class="my-4 flex justify-center gap-4">
        <Button variant="outline">View Assets</Button>
        <Button href="generate">Generate Assets</Button>
      </div>
    </div>
  </div>
</section>
