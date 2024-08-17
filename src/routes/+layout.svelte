<script>
  import ContextMenu from "$lib/components/ContextMenu.svelte";
  import { StatusInfo } from "$lib/records";
  import Toast from "$lib/Toast.svelte";
  import { onMount } from "svelte";

  import "../app.css";
  import "../icon.css";
  onMount(async () => {
    if (window.matchMedia && window.matchMedia("(prefers-color-scheme:dark)").matches) document.documentElement.setAttribute("data-theme", "dark");
  });
</script>

<main class="w-full h-dvh flex flex-col">
  <slot />
  <ContextMenu />
  <Toast />
</main>
