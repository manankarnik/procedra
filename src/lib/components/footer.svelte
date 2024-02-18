<script>
  import { page } from "$app/stores";
  let links = [
    { href: "/browse", text: "Browse" },
    { href: "/generate", text: "Generate" }
  ];
  if ($page.data.session) {
    links.push({ href: "/manage", text: "Manage" });
  }
  links.push({ href: "/dev", text: "For Devs" });
  links.push({ href: "https://github.com/manankarnik/procedra/releases", text: "Download" });
</script>

<footer class="bg-[hsl(var(--background))]/60 p-10 backdrop-blur">
  <div class="flex items-center justify-between gap-4">
    <div>
      <h3 class="text-xl font-bold sm:text-2xl">Procedra</h3>
    </div>
    <div class="hidden gap-4 md:grid md:grid-cols-3 lg:grid-cols-5">
      {#each links as link}
        <a
          href={link.href}
          class="text-md hidden text-center font-medium text-muted-foreground transition-colors hover:text-foreground sm:block"
        >
          {link.text}
        </a>
      {/each}
    </div>
    <div>
      <span class="my-2 font-bold">Manan Karnik</span>
      <div class="my-2 flex justify-center gap-4">
        <a href="https://github.com/manankarnik">
          <img
            src="https://www.svgrepo.com/show/512317/github-142.svg"
            class="h-5 w-5 dark:invert"
            alt="Github Logo"
          />
        </a>
        <a href="https://instagram.com/manankarnik">
          <img
            src="https://www.svgrepo.com/show/452229/instagram-1.svg"
            class="h-5 w-5"
            alt="Google Logo"
          />
        </a>
        <a href="https://x.com/manankarnik">
          <img
            src="https://upload.wikimedia.org/wikipedia/commons/5/53/X_logo_2023_original.svg"
            class="h-5 w-5 dark:invert"
            alt="Github Logo"
          />
        </a>
      </div>
    </div>
  </div>
</footer>
