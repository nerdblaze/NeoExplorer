<script>
  import StatusBar from "$lib/components/StatusBar.svelte";
  import ContextMenu from "$lib/components/ContextMenu.svelte";
  import "../app.css";
  import "../icon.css";
  import { StatusInfo } from "$lib/records";
</script>

<div class="w-full h-dvh flex flex-col">
  <slot />
  <ContextMenu/>
  <StatusBar />
</div>
