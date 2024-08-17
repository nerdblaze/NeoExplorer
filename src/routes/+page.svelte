<script>
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { WindowTabs } from "$lib/records";
  import { onDestroy, onMount } from "svelte";
  import { getAllWebviewWindows } from "@tauri-apps/api/webviewWindow";
  import { open_folder, new_tab, switch_tab, init_window } from "$lib/common";
  import Titlebar from "$lib/components/Titlebar.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";

  let activeTab = 0;

  // Function to delete a tab by index
  const deleteTab = async (index) => {
    $WindowTabs = $WindowTabs.filter((tab, i) => i !== index);
    activeTab = activeTab >= $WindowTabs.length ? Math.max($WindowTabs.length - 1, 0) : activeTab;
    if ($WindowTabs.length > 0) switch_tab(activeTab);
  };

  // Function to handle keyboard events
  const handle_keyboard = async (event) => {
    if (event.ctrlKey) {
      if (event.key === "w") {
        event.preventDefault();
        deleteTab(activeTab);
      } else if (event.key === "t") {
        event.preventDefault();
        new_tab();
      }
    }
  };

  // Initial setup
  onMount(async () => {
    // Add keyboard event listener on mount
    document.addEventListener("keydown", handle_keyboard);

    // Add initial tab on mount if no other tabs are present
    if ($WindowTabs.length < 1) await new_tab(); // skips during adding new tabs
    await switch_tab(0);

    const onInitTab = (event) => {
      open_folder(event.payload);
    };

    init_window(onInitTab);
  });

  // Remove keyboard event listener on destroy
  onDestroy(() => {
    document.removeEventListener("keydown", handle_keyboard);
  });

  onMount(async () => {});
</script>

<!-- Window Tab -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  id="user-area"
  class="flex flex-row grow"
>
  <div
    id="tab-container"
    class="flex flex-col w-full grow"
  >
    <div
      id="tab-head-container"
      class="flex flex-row justify-between w-full bg-secondarybackground"
    >
      <div
        id="tab-heads"
        class="flex flex-row px-2 h-10 min-w-64 text-sm/6 overflow-hidden"
      >
        {#each $WindowTabs as tab, idx}
          <a
            class="flex flex-row p-1 mt-2 max-w-64 w-64 rounded-t-md text-left cursor-pointer {tab.isActive ? 'bg-primarybackground' : ''} justify-between"
            href="/"
            on:click={() => switch_tab(idx)}
          >
            <span class="overflow-hidden text-nowrap">{$WindowTabs[idx].currentPath.slice(-1)[0] || "This PC"}</span>
            <span
              on:click|preventDefault={() => deleteTab(idx)}
              role="button"
              tabindex=""
            >
              <i class="icon icon-x text-2xs"></i>
            </span>
          </a>
        {/each}
        <a
          class="p-2 rounded-t-md"
          href="/"
        >
          <i
            class="icon icon-plus text-2xs"
            on:click|preventDefault={() => new_tab()}
          ></i>
        </a>
      </div>
      <Titlebar />
    </div>
    {#each $WindowTabs as tab, index}
      {#if tab.isActive}
        <svelte:component
          this={tab.component}
          tabIndex={index}
          currentPath={tab.currentPath}
        />
      {/if}
    {/each}
  </div>
</div>
<StatusBar />
