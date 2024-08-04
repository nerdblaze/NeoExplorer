<script>
  import { getCurrentWindow, Window, LogicalPosition } from "@tauri-apps/api/window";
  import { Webview } from "@tauri-apps/api/webview";
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
    appWindow.hide();
  };

  const new_window = async () => {
    // const webview = new Webview(appWindow, 'theUniqueLabel', { url: '/index.html' });
    // const appWindow = new Window('theUniqueLabel', {'url': '/index.html'});
  };

  let isWindowDragging = false;
  let windowPosition = {x:0, y:0, isLive: false};

  const startMove = async (event) => {
    const titleBarHead = document.querySelector("#tab-head-container");
    isWindowDragging = titleBarHead === event.target;
    
    const dragWindow = async (e) => {
      if (isWindowDragging) {
        if(windowPosition.isLive === false) {
          windowPosition = {x: event.clientX, y: event.clientY, isLive: true};
        }
        const curPosition = (await getCurrentWindow().outerPosition());
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
    document.addEventListener('keydown', handle_keyboard);

    // Add initial tab on mount if no other tabs are present
    if ($WindowTabs.length < 1) await addTab(); // skips during adding new tabs
    activateTab(0);
  });

  // Remove keyboard event listener on destroy
  onDestroy(() => {
    document.removeEventListener('keydown', handle_keyboard);
  });


  onMount(async () => {

  });
</script>

<!-- Window Tab -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div id="user-area" class="flex flex-row grow">
  <div id="tab-container" class="flex flex-col w-full grow">
    <div id="tab-head-container" class="flex flex-row justify-between w-full bg-secondarybackground cursor-move" on:mousedown={startMove}>
      <ul id="tab-heads" class="flex flex-row px-2 h-8 text-sm/6">
        {#each $WindowTabs as tab, idx}
          <li
            class="flex flex-row p-1 min-w-8 w-64 rounded-t-md text-left cursor-pointer {tab.isActive ? 'bg-primarybackground' : ''} justify-between"
            on:click={() => activateTab(idx)}
          >
            <span>{$WindowTabs[idx].currentPath.slice(-1)[0] || 'This PC'}</span>
            <span>
              <i class="icon icon-x text-2xs" on:click={() => deleteTab(idx)} role="button" aria-label="Close tab" tabindex=0></i>
            </span>
          </li>
        {/each}
        <li class="p-2 rounded-t-md cursor-pointer" on:click={addTab}>
          <i class="icon icon-plus text-2xs"></i>
        </li>
      </ul>
      <ul id="window-button-container" class="flex flex-row px-2 h-8 text-sm text-center">
        <li class="flex relative p-1 min-w-8 cursor-pointer items-center justify-center" on:click={new_window}>
          <i class="absolute icon icon-window text-xs"></i>
          <i class="absolute right-1 bottom-1 icon icon-circle-plus bg-primarybackground text-2xs"></i>
        </li>
        <li class="flex p-1 min-w-8 cursor-pointer items-center justify-center" on:click={minimize_window}><i class="icon icon-minus text-xs"></i></li>
        <li class="flex p-1 min-w-8 cursor-pointer items-center justify-center" on:click={maximize_window}><i class="icon icon-{isWindowed?'expand':'compress'} text-xs"></i></li>
        <li class="flex p-1 min-w-8 cursor-pointer items-center justify-center" on:click={close_window}><i class="icon icon-x text-xs"></i></li>
      </ul>
    </div>
    {#each $WindowTabs as tab, index}
      {#if tab.isActive}
        <svelte:component this={tab.component} tabIndex={activeTab} currentPath={tab.currentPath}/>
      {/if}
    {/each}
  </div>
</div>
