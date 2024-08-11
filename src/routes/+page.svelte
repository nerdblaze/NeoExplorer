<script>
  import { getCurrentWindow, Window, LogicalPosition } from "@tauri-apps/api/window";
  import { listen } from "@tauri-apps/api/event";
  import { Webview } from "@tauri-apps/api/webview";
  import { WindowTabs } from "$lib/records";
  import { onDestroy, onMount } from "svelte";
  import { getAllWebviewWindows } from "@tauri-apps/api/webviewWindow";
  import { open_folder, new_window, new_tab, switch_tab} from "$lib/common";

  let activeTab = 0;

  // Function to delete a tab by index
  const deleteTab = async (index) => {
    $WindowTabs = $WindowTabs.filter((tab, i) => i !== index);
    activeTab = activeTab >= $WindowTabs.length ? Math.max($WindowTabs.length - 1, 0) : activeTab;
    switch_tab(activeTab);
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

  // Window Handling
  // const pin_window = async (event) => {
  //   appWindow.setAlwaysOnTop();
  // };
  const appWindow = getCurrentWindow();
  let isWindowed = true;
  const minimize_window = async (event) => {
    appWindow.minimize();
  };

  const maximize_window = async (event) => {
    appWindow.toggleMaximize();
    isWindowed = await appWindow.isMaximized();
  };

  const close_window = async (event) => {
    let windows = getAllWebviewWindows();
    if (windows.length > 1) {
      appWindow.close();
    } else {
      appWindow.hide();
    }
  };

  let isWindowDragging = false;
  let windowPosition = { x: 0, y: 0, isLive: false };

  const startMove = async (event) => {
    const titleBarMover = document.querySelector("#window-mover");
    isWindowDragging = titleBarMover === event.target;

    const dragWindow = async (e) => {
      if (isWindowDragging) {
        if (windowPosition.isLive === false) {
          windowPosition = { x: event.clientX, y: event.clientY, isLive: true };
        }
        const curPosition = await getCurrentWindow().outerPosition();
        curPosition.x = curPosition.x - (windowPosition.x - e.clientX);
        curPosition.y = curPosition.y - (windowPosition.y - e.clientY);
        await getCurrentWindow().setPosition(curPosition);
      }
    };

    const stopMove = () => {
      isWindowDragging = false;
      windowPosition.isLive = false;
      window.removeEventListener("mousemove", dragWindow);
      window.removeEventListener("mouseup", stopMove);
    };

    window.addEventListener("mousemove", dragWindow);
    window.addEventListener("mouseup", stopMove);
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
      if (unlisten) unlisten();
    };

    let unlisten = await listen("initialize", onInitTab);
    setTimeout(() => {
      if (unlisten) {
        unlisten();
      }
    }, 500);
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
      <ul
        id="tab-heads"
        class="flex flex-row px-2 h-8 text-sm/6 overflow-hidden"
      >
        {#each $WindowTabs as tab, idx}
          <li
            class="flex flex-row p-1 max-w-64 w-64 rounded-t-md text-left cursor-pointer {tab.isActive ? 'bg-primarybackground' : ''} justify-between"
            on:click={() => switch_tab(idx)}
          >
            <span class="overflow-hidden text-nowrap">{$WindowTabs[idx].currentPath.slice(-1)[0] || "This PC"}</span>
            <span>
              <i
                class="icon icon-x text-2xs"
                on:click={() => deleteTab(idx)}
                role="button"
                aria-label="Close tab"
                tabindex="0"
              ></i>
            </span>
          </li>
        {/each}
        <li
          class="p-2 rounded-t-md cursor-pointer"
          on:click={()=>new_tab()}
        >
          <i class="icon icon-plus text-2xs"></i>
        </li>
      </ul>
      <div
        id="window-mover"
        class="w-16 cursor-move grow"
        on:mousedown={startMove}
      ></div>
      <ul
        id="window-button-container"
        class="flex flex-row px-2 h-8 text-sm text-center"
      >
        <li
          class="flex relative p-1 min-w-8 cursor-pointer items-center justify-center"
          on:click={() => new_window("")}
        >
          <i class="absolute icon icon-window text-xs"></i>
          <i class="absolute right-1 bottom-1 icon icon-circle-plus bg-primarybackground text-2xs"></i>
        </li>
        <li
          class="flex p-1 min-w-8 cursor-pointer items-center justify-center"
          on:click={minimize_window}
        >
          <i class="icon icon-minus text-xs"></i>
        </li>
        <li
          class="flex p-1 min-w-8 cursor-pointer items-center justify-center"
          on:click={maximize_window}
        >
          <i class="icon icon-{isWindowed ? 'expand' : 'compress'} text-xs"></i>
        </li>
        <li
          class="flex p-1 min-w-8 cursor-pointer items-center justify-center"
          on:click={close_window}
        >
          <i class="icon icon-x text-xs"></i>
        </li>
      </ul>
    </div>
    {#each $WindowTabs as tab, index}
      {#if tab.isActive}
        <svelte:component
          this={tab.component}
          tabIndex={activeTab}
          currentPath={tab.currentPath}
        />
      {/if}
    {/each}
  </div>
</div>
