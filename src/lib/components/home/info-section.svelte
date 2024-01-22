<script>
  import * as Card from "$lib/components/ui/card";
  import { Button } from "$lib/components/ui/button";
</script>

<section id="more">
  <div class="h-[800px] bg-gradient-to-b from-transparent via-background via-15%">
    <div class="container py-40 flex items-center justify-center gap-2">
      <div>
        <h2
          class="leading-loose my-4 scroll-m-20 py-2 text-xl font-extrabold tracking-tight sm:py-4 sm:text-2xl lg:text-3xl"
        >
          Built on top of
          <span class="rounded-lg bg-muted text-muted-foreground p-2">
            bevy_generative
          </span>
          using the Bevy Game Engine
        </h2>
        <div class="grid gap-4">
          <Card.Root
            class="animate-gradient h-full p-[2px] hover:border-transparent hover:bg-gradient-to-r"
          >
            <div class="flex h-full flex-col justify-between rounded-md bg-background">
              <Card.Header>
                <Card.Title>bevy_generative</Card.Title>
                <Card.Description>
                  A procedural generation plugin written for the bevy game engine
                  with the procedra web app in mind.
                </Card.Description>
              </Card.Header>
              <Card.Content class="flex gap-4 flex-col gap-4 sm:flex-row">
                <a href="https://github.com/manankarnik/bevy_generative">
                  <Button class="w-full" variant="primary">Github</Button>
                </a>
                <a href="https://crates.io/crates/bevy_generative">
                  <Button class="w-full" variant="primary">Crates.io</Button>
                </a>
              </Card.Content>
            </div>
          </Card.Root>
          <Card.Root
            class="animate-gradient h-full p-[2px] hover:border-transparent hover:bg-gradient-to-r"
          >
            <div class="flex h-full flex-col justify-between rounded-md bg-background">
              <Card.Header>
                <Card.Title>Bevy</Card.Title>
                <Card.Description>
                  A refreshingly simple data-driven game engine built in Rust.
                </Card.Description>
              </Card.Header>
              <Card.Content class="flex flex-col gap-4 sm:flex-row">
                <a href="https://bevyengine.org">
                  <Button class="w-full" variant="primary">Website</Button>
                </a>
                <a href="https://github.com/bevyengine">
                  <Button class="w-full" variant="primary">Github</Button>
                </a>
                <a href="https://crates.io/crates/bevy">
                  <Button class="w-full" variant="primary">Crates.io</Button>
                </a>
              </Card.Content>
            </div>
          </Card.Root>
          <Card.Root
            class="animate-gradient h-full p-[2px] hover:border-transparent hover:bg-gradient-to-r"
          >
            <div class="flex h-full flex-col justify-between rounded-md bg-background">
              <Card.Header>
                <Card.Title>Rust</Card.Title>
                <Card.Description>
                  A programming language empowering everyone to build reliable and efficient
                  software. Both Bevy and bevy_generative are built in Rust.
                </Card.Description>
              </Card.Header>
              <Card.Content class="flex flex-col gap-4 sm:flex-row">
                <a href="https://rust-lang.org">
                  <Button class="w-full" variant="primary">Website</Button>
                </a>
                <a href="https://github.com/rust-lang">
                  <Button class="w-full" variant="primary">Github</Button>
                </a>
              </Card.Content>
            </div>
          </Card.Root>
          <div>
            <p></p>
          </div>
        </div>
      </div>
    </div>
  </div>
</section>
