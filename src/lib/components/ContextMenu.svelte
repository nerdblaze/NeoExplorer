<script>
  import { ContextMenuList } from "$lib/records";
  import { hideContextMenu } from "$lib/common";
  import { onMount } from "svelte";

  onMount(() => {
    document.addEventListener("click", hideContextMenu);
  });
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<div
  id="context-menu-container"
  class="absolute z-[9999] bg-primarybackground border-hinttext border rounded-lg shadow-lg max-w-[200px] {$ContextMenuList.visible ? 'block' : 'hidden'}"
  style="left: {$ContextMenuList.x}px; top: {$ContextMenuList.y}px;"
>
  <ul class="p-0 m-0 list-none">
    {#each $ContextMenuList.items as item}
      <li class="px-4 py-2 cursor-pointer hover:bg-secondarybackground rounded-md" on:click|preventDefault={()=>item.callback(...item.args)}>
        {item.label}
      </li>
    {/each}
  </ul>
</div>