<script>
  import Titlebar from "$lib/components/Titlebar.svelte";
  let title = "Settings";
</script>

<Titlebar {title}/>
<div
  id="settings-main-container"
  class="flex h-full overflow-hidden"
>
  <slot />
</div>

<style>
</style>
