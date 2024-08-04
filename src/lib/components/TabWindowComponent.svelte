<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onDestroy, onMount } from "svelte";
  import DriveItem from "./DriveItem.svelte";
  import { notify } from "$lib/utilities";
  import { WindowTabs, StatusInfo } from "$lib/records";
  import ListViewItem from "./ListViewItem.svelte";

  export let tabIndex = 0;

  // Component state
  let width = 200;
  let isResizing = false;
  let search_term = "";
  let drives = [];
  let items = [];
  let oldPath = [];
  let search_results = [];
  let breadcrumbsVisible = false;

  // Update breadcrumbs visibility based on current path
  $: breadcrumbsVisible = $WindowTabs[tabIndex].currentPath.length !== 0;

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
      if (search_term) {
        search_results = await invoke("search_system", { searchTerm: search_term, page: 1, page_length: 1000 });
      }
    } else if (event.key === "Backspace") {
      goBack();
    }
  };

  // Handle clicks outside breadcrumb or search input
  function handleClickOutside(event) {
    const breadcrumbContainer = document.getElementById("breadcrumb-container");
    const searchContainer = document.getElementById("tab-quickbar-search");
    const inputBox = document.getElementById("search-input");

    if ((breadcrumbContainer) && !breadcrumbContainer.contains(event.target) && searchContainer.contains(event.target)) {
      breadcrumbsVisible = false;
      if (inputBox) inputBox.focus();
    } else if ((inputBox) && (!inputBox.contains(event.target))) {
      breadcrumbsVisible = true;
    }
  }

  // Mount component: load drives and set up event listeners
  onMount(async () => {
    if($WindowTabs[tabIndex].currentPath.length == 0){
      drives = await invoke("list_drives", {});
      items = [];
    } else {
      reloadPage();
    }
    
    document.addEventListener("click", handleClickOutside);
    document.addEventListener("keydown", handleKeyboard);
    console.log("Loaded");
  });

  // Clean up event listeners on component destroy
  onDestroy(() => {
    document.removeEventListener("click", handleClickOutside);
    document.removeEventListener("keydown", handleKeyboard);
  });

  // Navigate back in folder history
  async function goBack() {
    const folder = $WindowTabs[tabIndex].currentPath.pop();
    if (folder) oldPath.push(folder);
    $WindowTabs[tabIndex].currentPath = [...$WindowTabs[tabIndex].currentPath];
    if ($WindowTabs[tabIndex].currentPath.length > 0) {
      let path = $WindowTabs[tabIndex].currentPath.join("\\");
      await openFolder(path);
    }
  }

  // Reload the current folder
  async function reloadPage() {
    if ($WindowTabs[tabIndex].currentPath.length > 0) {
      let path = $WindowTabs[tabIndex].currentPath.join("\\");
      await openFolder(path);
    }
  }

  // Navigate forward in folder history
  async function goFront() {
    const folder = oldPath.pop();
    if (folder) $WindowTabs[tabIndex].currentPath = [...$WindowTabs[tabIndex].currentPath, folder];
    if ($WindowTabs[tabIndex].currentPath.length > 0) {
      let path = $WindowTabs[tabIndex].currentPath.join("\\");
      await openFolder(path);
    }
  }

  // Open a folder or file item
  async function openItem(itemPath, is_folder = true, is_drive = false) {
    if (is_drive) {
      $WindowTabs[tabIndex].currentPath = [];
    }

    if (is_folder) {
      $WindowTabs[tabIndex].currentPath = [...$WindowTabs[tabIndex].currentPath, itemPath];
      let path = $WindowTabs[tabIndex].currentPath.join("\\");
      await openFolder(path);
    } else {
      console.log("Opened file " + itemPath);
    }
  }

  // Open a folder and handle potential errors
  async function openFolder(folderPath) {
    try {
      search_term = "";
      const response = await invoke("open_folder", { folderPath });
      items = response;
      $StatusInfo.file_count = items.length;
    } catch (error) {
      notify(error, "error");
      $WindowTabs[tabIndex].currentPath = [...$WindowTabs[tabIndex].currentPath.slice(0, -1)];
    }
  }

  // Handle breadcrumb link click
  async function handleBreadcrumbClick(index) {
    $WindowTabs[tabIndex].currentPath = [...$WindowTabs[tabIndex].currentPath.slice(0, index + 1)];
    if ($WindowTabs[tabIndex].currentPath.length !== 0) {
      await openFolder($WindowTabs[tabIndex].currentPath.join("\\"));
    }
  }
  async function openSearchResult(item){
    
    if(item.file_attributes.directory){
      $WindowTabs[tabIndex].currentPath = [...item.file_path.split("\\")]
      reloadPage();

    }
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div id="tab-quickbar-container" class="h-8 w-full flex items-center justify-between bg-primarybackground my-2 mx-auto">
  <ul id="tab-quickbar-nav" class="flex items-center space-x-2">
    <li class="w-6 h-6 flex items-center justify-center text-primarytext cursor-pointer hover:text-accentprimary">
      <i class="icon icon-angle-left text-xs" on:click={goBack}></i>
    </li>
    <li class="w-6 h-6 flex items-center justify-center text-primarytext cursor-pointer hover:text-accentprimary">
      <i class="icon icon-angle-right text-xs" on:click={goFront}></i>
    </li>
    <li class="w-6 h-6 flex items-center justify-center text-primarytext cursor-pointer hover:text-accentprimary">
      <i class="icon icon-arrow-rotate-right text-xs"></i>
    </li>
  </ul>

  <ul id="tab-quickbar-search" class="flex-grow mx-4">
    {#if breadcrumbsVisible}
      <!-- Breadcrumbs Section -->
      <div id="breadcrumb-container" class="flex items-center space-x-2 w-fit">
        <span>
          <a class="hover:underline hover:cursor-pointer" href="/" on:click={() => handleBreadcrumbClick(-1)}>
            This PC
          </a>
          <i class="icon icon-angle-right mx-2 text-2xs"></i>
        </span>

        {#each $WindowTabs[tabIndex].currentPath as part, index}
          <span>
            <a class="hover:underline hover:cursor-pointer" href="/" on:click={() => handleBreadcrumbClick(index)}>
              {part}
            </a>
            {#if index < $WindowTabs[tabIndex].currentPath.length - 1}
              <i class="icon icon-angle-right mx-2 text-2xs"></i>
            {/if}
          </span>
        {/each}
      </div>
    {:else}
      <!-- Search Bar Section -->
      <input
        id="search-input"
        type="text"
        class="w-full h-8 bg-surfacebackground border border-hinttext rounded-lg px-3 focus:outline-none focus:border-accentprimary"
        placeholder="🔍 Search..."
        bind:value={search_term}
      />
    {/if}
  </ul>

  <ul id="tab-quickbar-tool" class="flex items-center space-x-2">
    <li class="w-6 h-6 flex items-center justify-center text-primarytext cursor-pointer hover:text-accentprimary">
      <i class="icon icon-arrow-up-arrow-down text-xs"></i>
    </li>
    <li class="w-6 h-6 flex items-center justify-center text-primarytext cursor-pointer hover:text-accentprimary">
      <i class="icon icon-filter text-xs"></i>
    </li>
    <li class="w-6 h-6 flex items-center justify-center text-primarytext cursor-pointer hover:text-accentprimary">
      <i class="icon icon-list text-xs"></i>
    </li>
    <li class="w-6 h-6 flex items-center justify-center text-primarytext cursor-pointer hover:text-accentprimary">
      <i class="icon icon-sidebar-flip text-xs"></i>
    </li>
  </ul>
</div>

<div id="tab-window-container" class="flex h-full">
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div id="tree-view-container" class="flex flex-col border-r border-dividerline" style="width: {width}px;" on:mousedown={startResize}></div>

  <div id="explorer-view-container" class="flex-1 p-4 overflow-y-auto overflow-x-hidden">
    {#if search_term}
      <ul id="search-result-list" class="flex flex-row flex-wrap p-2">
        {#if search_results.length === 0}
          <li class="text-primarytext">No result found</li>
        {/if}

        {#each search_results as item}
          <ListViewItem {item} on:click={() => openSearchResult(item)}/>
        {/each}
      </ul>
    {:else}
      {#if $WindowTabs[tabIndex].currentPath.length === 0}
        <ul id="drive-list" class="flex flex-row p-2 flex-wrap">
          {#each drives as drive}
            <DriveItem {drive} on:click={() => openItem(`${drive.disk_label}:`, true)} />
          {/each}
        </ul>
      {:else}
        <ul id="file-list" class="flex flex-row flex-wrap p-2">
          {#if items.length === 0}
            <li class="text-primarytext">Empty Folder</li>
          {/if}

          {#each items as item}
            <ListViewItem {item} on:click={() => openItem(`${item.file_name}`, item.file_attributes.directory)}/>
          {/each}

        </ul>
      {/if}
    {/if}
  </div>

  <div id="file-preview-container" class="hidden flex-1 bg-secondarybackground">
    <!-- File preview content -->
  </div>
</div>

<style>
  #explorer-view-container {
    height: calc(100vh - 8rem); /* Adjust based on your header and other UI elements */
    scrollbar-width: thin;
  }
</style>
