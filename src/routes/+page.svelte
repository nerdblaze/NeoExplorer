<script>
  import TabWindowComponent from "$lib/components/TabWindowComponent.svelte";
  import { WindowTabs } from "$lib/records";
  import { onDestroy, onMount } from "svelte";

  let activeTab = 0;

  // Function to add a new tab
  async function addTab() {
    $WindowTabs = [
      ...$WindowTabs,
      {
        component: TabWindowComponent,
        isActive: false, // Track if this tab is active
        currentPath: [],
      },
    ];
    activateTab($WindowTabs.length - 1);
  }

  // Function to delete a tab by index
  function deleteTab(index) {
    $WindowTabs = $WindowTabs.filter((tab, i) => i !== index);
    activeTab = activeTab >= $WindowTabs.length ? Math.max($WindowTabs.length - 1, 0) : activeTab;
    activateTab(activeTab);
  }

  // Function to activate a tab
  function activateTab(index) {
    $WindowTabs = $WindowTabs.map((tab, i) => ({
      ...tab,
      isActive: i === index,
    }));
    activeTab = index;
  }

  // Function to handle keyboard events
  const handle_keyboard = (event) => {
    if (event.ctrlKey){
      if(event.key === 'w') {
        event.preventDefault();
        deleteTab(activeTab);
      }
    }
  };

  // Add keyboard event listener on mount
  onMount(() => {
    document.addEventListener('keydown', handle_keyboard);
  });

  // Remove keyboard event listener on destroy
  onDestroy(() => {
    document.removeEventListener('keydown', handle_keyboard);
  });

  // Initial setup
  onMount(async () => {
    if ($WindowTabs.length < 1) await addTab(); // Add initial tab on mount
    activateTab(0);
  });
</script>

<!-- Window Tab -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<div id="user-area" class="flex flex-row grow">
  <div id="tab-container" class="flex flex-col w-full grow">
    <div id="tab-head-container" class="flex flex-row justify-between w-full bg-secondarybackground">
      <ul id="tab-heads" class="flex flex-row px-2 h-8 text-sm/6">
        {#each $WindowTabs as tab, idx}
          <li
            class="p-1 min-w-8 w-24 rounded-t-md text-left cursor-pointer {tab.isActive ? 'bg-primarybackground' : ''} flex flex-row justify-between"
            on:click={() => activateTab(idx)}
          >
            <span>{$WindowTabs[activeTab].currentPath.slice(-1)[0] || 'This PC'}</span>
            <span>
              <i class="icon icon-x text-2xs" on:click={() => deleteTab(idx)} role="button" aria-label="Close tab" tabindex=0></i>
            </span>
          </li>
        {/each}
        <li class="p-2 rounded-t-md" on:click={addTab}>
          <i class="icon icon-plus text-2xs"></i>
        </li>
      </ul>
      <ul id="window-button-container" class="flex flex-row px-2 h-8 text-sm text-center">
        <li class="p-1 min-w-8 cursor-pointer"><i class="icon icon-minus text-xs"></i></li>
        <li class="p-1 min-w-8 cursor-pointer"><i class="icon icon-square text-xs"></i></li>
        <li class="p-1 min-w-8 cursor-pointer"><i class="icon icon-x text-xs"></i></li>
      </ul>
    </div>
    {#each $WindowTabs as tab, index}
      {#if tab.isActive}
        <svelte:component this={tab.component} tabIndex={activeTab} currentPath={tab.currentPath}/>
      {/if}
    {/each}
  </div>
</div>
