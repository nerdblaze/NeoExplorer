<script>
  import { WindowTabs, ContextMenuList } from "$lib/records.js";
  import { createContextMenu, open_folder, open_file, new_window, new_tab } from "$lib/common.js";
  import { get_active_tab, notify, formatBytes, formatDates, sort_array } from "$lib/utilities.js";

  const activeTab = get_active_tab();

  const handleEvent = async (e, item, action) => {
    e.preventDefault();
    switch (action) {
      case "open":
        let path = "";
        if ($WindowTabs[activeTab].searchTerm) {
          path = item.file_path;
        } else {
          let pathbuff = [...$WindowTabs[activeTab].currentPath];
          pathbuff.push(item.file_name);
          path = pathbuff.join("\\");
        }
        item.file_attributes.directory ? open_folder(path) : open_file(path);
        break;
      case "properties":
        new_window("/properties", item.file_path);
        break;
      case "delete":
        notify("#TODO[Dummy ContextMenu]", "Warning");
        break;
      case "open_in_tab":
        if (item.file_attributes.directory) {
          new_tab(item.file_path);
        }
        break;
      case "open_in_window":
        if (item.file_attributes.directory) {
          new_window("/", item.file_path);
        }
        break;
      default:
        break;
    }
  };

  const show_context_options = (e, item) => {
    if (item.file_attributes.directory) {
      $ContextMenuList.items = [
        { label: "Open", callback: handleEvent, args: [e, item, "open"] },
        { label: "Open in new tab", callback: handleEvent, args: [e, item, "open_in_tab"] },
        { label: "Open in new window", callback: handleEvent, args: [e, item, "open_in_window"] },
        { label: "Delete", callback: handleEvent, args: [e, item, "delete"] },
        { label: "Properties", callback: handleEvent, args: [e, item, "properties"] },
      ];
    } else {
      $ContextMenuList.items = [
        { label: "Open", callback: handleEvent, args: [e, item, "open"] },
        { label: "Delete", callback: handleEvent, args: [e, item, "delete"] },
        { label: "Properties", callback: handleEvent, args: [e, item, "properties"] },
      ];
    }
  };

  let sortField = "";
  let sortAsc = true;

  // Handle column sorting
  const handle_sort = async (field) => {
    if (sortField === field) {
      sortAsc = !sortAsc;
    } else {
      sortField = field;
      sortAsc = false;
    }
    $WindowTabs[activeTab].currentView = [...sort_array($WindowTabs[activeTab].currentView, sortField, sortAsc)];
  };
</script>

<!-- Tile View -->
{#if $WindowTabs[activeTab].viewMode == 0}
  <ul
    id="file-list"
    class="flex flex-row flex-wrap p-2"
  >
    {#if $WindowTabs[activeTab].currentView.length === 0}
      <li class="text-primarytext">Empty</li>
    {/if}

    {#each $WindowTabs[activeTab].currentView as item, idx}
      <li class="flex items-center p-2 min-w-64 w-full sm:w-1/2 md:w-1/4 lg:w-1/6 hover:bg-surfacebackground rounded-md overflow-hidden">
        <button
          class="flex items-center"
          on:dblclick|preventDefault={(e) => handleEvent(e, item, "open")}
          on:contextmenu|preventDefault={(e) => createContextMenu(e, show_context_options, item)}
          tabindex={idx}
        >
          <i class="flex justify-center items-center w-16 h-16 icon icon-{item.file_attributes.directory ? 'folder' : 'file'} cursor-pointer text-3xl"></i>
          <ul class="flex flex-col text-xs">
            <span class="flex w-full text-nowrap">{item.file_name}{item.file_name.length > 30 ? "..." : ""}</span>
            {#if !item.file_attributes.directory}
              <span class="flex">{item.file_ext} file</span>
              <span class="flex">{formatBytes(item.file_size)}</span>
            {/if}
          </ul>
        </button>
      </li>
    {/each}
  </ul>
  <!-- List View -->
{:else if $WindowTabs[activeTab].viewMode == 1}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <button class="flex flex-row items-center w-full">
    <span
      class="flex px-2 py-1 w-full overflow-hidden text-wrap text-left"
      on:click={() => handle_sort("file_name")}
    >
      <i
        class="icon text-xs p-1"
        class:icon-arrow-down-small-big={sortField === "file_name" && sortAsc}
        class:icon-arrow-up-big-small={sortField === "file_name" && !sortAsc}
      ></i>
      Name
    </span>
    <span
      class="flex px-2 py-1 w-1/5 overflow-hidden text-wrap items-center"
      on:click={() => handle_sort("file_modification_time")}
    >
      <i
        class="icon text-xs p-1"
        class:icon-arrow-down-small-big={sortField === "file_modification_time" && sortAsc}
        class:icon-arrow-up-big-small={sortField === "file_modification_time" && !sortAsc}
      ></i>
      Date modified
    </span>
    <span
      class="flex px-2 w-2/12 overflow-hidden text-wrap items-center"
      on:click={() => handle_sort("file_ext")}
    >
      <i
        class="icon text-xs p-1"
        class:icon-arrow-down-small-big={sortField === "file_ext" && sortAsc}
        class:icon-arrow-up-big-small={sortField === "file_ext" && !sortAsc}
      ></i>

      Type
    </span>
    <span
      class="flex px-1 w-2/12 text-wrap items-center"
      on:click={() => handle_sort("file_size")}
    >
      Size
      <i
        class="icon text-xs p-1"
        class:icon-arrow-down-small-big={sortField === "file_size" && sortAsc}
        class:icon-arrow-up-big-small={sortField === "file_size" && !sortAsc}
      ></i>
    </span>
  </button>
  <ul
    id="file-list"
    class="flex flex-col flex-wrap p-2 w-full"
  >
    {#if $WindowTabs[activeTab].currentView.length === 0}
      <li class="text-primarytext">Empty</li>
    {/if}

    {#each $WindowTabs[activeTab].currentView as item}
      <li class="flex items-center p-2 w-full overflow-hidden hover:bg-surfacebackground rounded-md">
        <button
          class="flex items-center w-full"
          on:dblclick|preventDefault={(e) => handleEvent(e, item, "open")}
          on:contextmenu|preventDefault={(e) => createContextMenu(e, show_context_options, item)}
          tabindex="0"
        >
          <i class="flex justify-center items-center w-8 h-8 icon icon-{item.file_attributes.directory ? 'folder' : 'file'} cursor-pointer text-lg"></i>
          <ul class="flex flex-row text-sm w-full h-10 justify-between">
            <span class="flex px-2 py-1 w-full overflow-hidden text-wrap text-left">{item.file_name}</span>
            <span class="flex px-2 py-1 w-1/5 overflow-hidden text-wrap items-center">{formatDates(item.file_modification_time)}</span>
            <span class="flex px-2 w-2/12 overflow-hidden text-wrap items-center">
              {#if !item.file_attributes.directory}
                {item.file_ext} file
              {:else}
                Folder
              {/if}
            </span>
            <span class="flex px-1 w-2/12 text-wrap items-center">
              {#if !item.file_attributes.directory}
                {formatBytes(item.file_size)}
              {/if}
            </span>
          </ul>
        </button>
      </li>
    {/each}
  </ul>
{/if}

<style>
</style>
