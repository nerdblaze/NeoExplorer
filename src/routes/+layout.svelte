<script>
  import StatusBar from "$lib/components/StatusBar.svelte";
  import ContextMenu from "$lib/components/ContextMenu.svelte";
  import Titlebar from "$lib/components/Titlebar.svelte";
  import { StatusInfo } from "$lib/records";
  import Toast from "$lib/Toast.svelte";

  import "../app.css";
  import "../icon.css";
</script>

<main class="w-full h-dvh flex flex-col">
  <slot />
  <StatusBar />
  <ContextMenu />
  <Toast />
</main>
