<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onDestroy, onMount } from "svelte";
  import DriveView from "./DriveView.svelte";
  import { get_active_tab, notify } from "$lib/utilities";
  import { open_folder, search_system } from "$lib/common";
  import { WindowTabs, StatusInfo } from "$lib/records";
  import ExplorerView from "./ExplorerView.svelte";

  export let tabIndex = 0;

  // Component state
  let width = 200;
  let isResizing = false;
  let search_results = [];
  let breadcrumbsVisible = true;

  // Start resizing handler
  const startResize = (event) => {
    isResizing = Math.abs(event.clientX - event.target.getBoundingClientRect().right) <= 16;

    const resize = (e) => {
      if (isResizing) {
        width = e.clientX - event.target.getBoundingClientRect().left;
      }
    };

    const stopResize = () => {
      isResizing = false;
      window.removeEventListener("mousemove", resize);
      window.removeEventListener("mouseup", stopResize);
    };

    window.addEventListener("mousemove", resize);
    window.addEventListener("mouseup", stopResize);
  };

  // Handle keyboard input for search and navigation
  const handleKeyboard = async (event) => {
    if (event.ctrlKey) {
      if (event.key === "r") {
        reloadPage();
        event.preventDefault();
      }
    } else if (event.key === "Enter") {
      event.preventDefault();
      search_system($WindowTabs[tabIndex].searchTerm);
    } else if (event.key === "Backspace") {
      if (event.target.nodeName === "INPUT") return;
      goBack();
    }
  };

  // Handle clicks outside breadcrumb or search input
  function handleClickOutside(event) {
    const breadcrumbContainer = document.querySelector("#breadcrumb-container");
    const searchContainer = document.querySelector("#tab-quickbar-search");
    const inputBox = document.querySelector("#search-input");

    if (breadcrumbContainer && !breadcrumbContainer.contains(event.target) && searchContainer.contains(event.target)) {
      breadcrumbsVisible = false;
      if (inputBox) inputBox.focus();
    } else if (inputBox && !inputBox.contains(event.target)) {
      breadcrumbsVisible = true;
    }
  }

  // Mount component: load drives and set up event listeners
  onMount(async () => {
    document.addEventListener("click", handleClickOutside);
    document.addEventListener("keydown", handleKeyboard);
  });

  // Clean up event listeners on component destroy
  onDestroy(async () => {
    document.removeEventListener("click", handleClickOutside);
    document.removeEventListener("keydown", handleKeyboard);
  });

  // Navigate back in folder history
  async function goBack() {
    const pathbuff = $WindowTabs[tabIndex].currentPath;
    let path = pathbuff.join("\\");
    $WindowTabs[tabIndex].oldPath.push(path);
    pathbuff.pop();
    path = pathbuff.join("\\");
    open_folder(path);
  }

  // Navigate forward in folder history
  async function goFront() {
    const path = $WindowTabs[tabIndex].oldPath.pop();
    if (path) open_folder(path);
  }

  // Reload the current folder
  async function reloadPage() {
    const tabIndex = await get_active_tab();
    if ($WindowTabs[tabIndex].currentPath.length > 0) {
      let path = $WindowTabs[tabIndex].currentPath.join("\\");
      await open_folder(path);
    }
  }

  // Handle breadcrumb link click
  async function handleBreadcrumbClick(index) {
    let pathbuff = [...$WindowTabs[tabIndex].currentPath.slice(0, index + 1)];
    await open_folder(pathbuff.join("\\"));
  }
  async function openSearchResult(item) {
    if (item.file_attributes.directory) {
      $WindowTabs[tabIndex].currentPath = [...item.file_path.split("\\")];
      reloadPage();
    }
  }
  const toggle_view = async () => {
    $WindowTabs[tabIndex].viewMode = ($WindowTabs[tabIndex].viewMode + 1) % 2;
  };
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<div
  id="tab-quickbar-container"
  class="h-10 w-full flex items-center justify-between bg-primarybackground my-1 pb-1 mx-auto shadow-sm shadow-dividerline"
>
  <ul
    id="tab-quickbar-nav"
    class="flex items-center space-x-2"
  >
    <li
      class="w-6 h-6 flex items-center justify-center text-primarytext cursor-pointer hover:text-accentprimary"
      on:click={goBack}
    >
      <i class="icon icon-angle-left text-xs"></i>
    </li>
    <li
      class="w-6 h-6 flex items-center justify-center text-primarytext cursor-pointer hover:text-accentprimary"
      on:click={goFront}
    >
      <i class="icon icon-angle-right text-xs"></i>
    </li>
    <li
      class="w-6 h-6 flex items-center justify-center text-primarytext cursor-pointer hover:text-accentprimary"
      on:click={reloadPage}
    >
      <i class="icon icon-arrow-rotate-right text-xs"></i>
    </li>
  </ul>

  <div
    id="tab-quickbar-search"
    class="flex flex-grow mx-4 px-2 py-1 text-nowrap overflow-x-scroll bg-surfacebackground rounded-lg"
  >
    {#if breadcrumbsVisible}
      <!-- Breadcrumbs Section -->
      <ul
        id="breadcrumb-container"
        class="flex space-x-2 w-fit"
      >
        <li class="flex">
          <a
            class="hover:underline hover:cursor-pointer"
            href="/"
            on:click={() => handleBreadcrumbClick(-1)}
          >
            This PC
          </a>
          <i class="icon icon-angle-right mx-2 text-2xs self-center"></i>
        </li>

        {#each $WindowTabs[tabIndex].currentPath as part, index}
          <li class="flex items-center">
            <a
              class="hover:underline hover:cursor-pointer"
              href="/"
              on:click={() => handleBreadcrumbClick(index)}
            >
              {part}
            </a>
            {#if index < $WindowTabs[tabIndex].currentPath.length - 1}
              <i class="icon icon-angle-right mx-2 text-2xs"></i>
            {/if}
          </li>
        {/each}
      </ul>
    {:else}
      <!-- Search Bar Section -->
       <i class="flex icon icon-magnifying-glass text-2xs items-center"></i>
      <input
        id="search-input"
        type="text"
        class="w-full h-6 bg-surfacebackground mx-2 py-1 focus:outline-none focus:border-accentprimary"
        placeholder="Search..."
        bind:value={$WindowTabs[tabIndex].searchTerm}
      />
    {/if}
  </div>

  <ul
    id="tab-quickbar-tool"
    class="flex items-center space-x-2"
  >
    <li class="w-6 h-6 flex items-center justify-center text-primarytext cursor-pointer hover:text-accentprimary">
      <i class="icon icon-arrow-up-arrow-down text-xs"></i>
    </li>
    <li class="w-6 h-6 flex items-center justify-center text-primarytext cursor-pointer hover:text-accentprimary">
      <i class="icon icon-filter text-xs"></i>
    </li>
    <li
      class="w-6 h-6 flex items-center justify-center text-primarytext cursor-pointer hover:text-accentprimary"
      on:click={toggle_view}
    >
      <i class="icon icon-list text-xs"></i>
    </li>
    <li class="w-6 h-6 flex items-center justify-center text-primarytext cursor-pointer hover:text-accentprimary">
      <i class="icon icon-sidebar-flip text-xs"></i>
    </li>
  </ul>
</div>

<div
  id="tab-window-container"
  class="flex h-full"
>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    id="tree-view-container"
    class="hidden md:flex flex-col border-r border-dividerline"
    style="width: {width}px;"
    on:mousedown={startResize}
  ></div>

  <div
    id="explorer-view-container"
    class="flex-1 p-4 overflow-y-auto overflow-x-hidden"
  >
    {#if $WindowTabs[tabIndex].currentView.length === 0}
      <DriveView />
    {:else}
      <ExplorerView />
    {/if}
  </div>

  <div
    id="file-preview-container"
    class="hidden flex-1 bg-secondarybackground"
  >
    <!-- File preview content -->
  </div>
</div>

<style>
  #explorer-view-container {
    height: calc(100vh - 8rem); /* Adjust based on your header and other UI elements */
    scrollbar-width: thin;
    scrollbar-track-color: transparent;
  }
  #tab-quickbar-search {
    scrollbar-width: none;
  }
</style>
