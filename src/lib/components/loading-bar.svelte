<script>
  import { beforeNavigate, afterNavigate } from "$app/navigation";
  let progress = "w-0";
  let display = "hidden";
  beforeNavigate(() => {
    display = "visible";
    setTimeout(() => {
      progress = "w-[40%]";
    }, 0);
  });
  afterNavigate(() => {
    progress = "w-full";
    setTimeout(() => {
      display = "hidden";
      progress = "w-0";
    }, 200);
  });
</script>

<div
  class={`animate-gradient absolute top-0 z-20 h-1 w-0 bg-gradient-to-r transition-[width] duration-200 ${progress} ${display}`}
/>
