<script>
  import { ContextMenuList } from "$lib/records";
  import { hideContextMenu } from "$lib/common";
  import { onMount } from "svelte";

  onMount(() => {
    document.addEventListener("click", hideContextMenu);
  });
</script>

<div
  id="context-menu-container"
  class="{$ContextMenuList.visible ? 'block' : 'hidden'} absolute max-w-52 z-[9999] bg-primarybackground border-hinttext border rounded-lg shadow-lg text-sm"
  style="left: {$ContextMenuList.x}px; top: {$ContextMenuList.y}px;"
>
  <ul class="p-0 m-0 list-none">
    {#each $ContextMenuList.items as item}
      <li class="px-2 py-1 cursor-pointer hover:bg-secondarybackground rounded-lg" on:click|preventDefault={()=>item.callback(...item.args)}>
        {item.label}
      </li>
    {/each}
  </ul>
</div>