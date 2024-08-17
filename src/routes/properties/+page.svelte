<script>
  import { onDestroy, onMount } from "svelte";
  import { init_window, get_properties } from "$lib/common";

  let metadata = {};
  let selectedCategory = "General";

  onMount(async () => {
    const initFunc = async (event) => {
      metadata = await get_properties(event.payload);
    };
    init_window(initFunc);
  });
  onDestroy(() => {});

  function handleCategoryClick(category) {
    selectedCategory = category;
  }
</script>

<div
  id="properties-page-container"
  class="flex w-full overflow-y-scroll"
>
  <!-- Sidebar -->
  <aside id="properties-sidebar-container" class="w-1/4 p-4 shadow-sm shadow-dividerline">
    <ul class="flex flex-col sticky top-0">
      {#each Object.keys(metadata.categories || {}) as category}
        <li class="mb-2">
          <button
            on:click={() => handleCategoryClick(category)}
            class="w-full text-left py-2 px-4 rounded hover:bg-surfacebackground uppercase"
            class:font-bold={category === selectedCategory}
          >
            {category}
          </button>
        </li>
      {/each}
    </ul>
  </aside>

  <!-- Main Content -->
  <div id="properties-main-container" class="w-3/4 p-4">
    {#if metadata.categories && metadata.categories[selectedCategory]}
      <div>
        <ul>
          {#each Object.entries(metadata.categories[selectedCategory]) as [key, value]}
            <li class="py-1">
              <span class="font-medium capitalize">{key}:</span>
              {value}
            </li>
          {/each}
        </ul>
      </div>
    {:else}
      <p class="">No metadata available for this category.</p>
    {/if}
  </div>
</div>

<style>
  #properties-page-container {
    scrollbar-width: none;
  }
</style>
