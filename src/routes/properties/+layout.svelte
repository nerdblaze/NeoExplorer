<script>
  import Titlebar from "$lib/components/Titlebar.svelte";
  let title = "File Information"
</script>

<Titlebar {title}/>
<div
  id="properties-main-container"
  class="flex h-full w-full overflow-hidden"
>
  <slot />
</div>

<style>
</style>
