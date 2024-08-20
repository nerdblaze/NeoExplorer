<script>
  import { WindowTabs, ContextMenuList } from "$lib/records.js";
  import { createContextMenu, open_folder, open_file, new_window, new_tab, reload_page, delete_item, new_file, new_folder } from "$lib/common.js";
  import { get_active_tab, notify, formatBytes, formatDates, sort_array } from "$lib/utilities.js";

  const activeTab = get_active_tab();

  const handleEvent = async (e, item, action) => {
    e.preventDefault();
    let path = "";
    switch (action) {
      case "open":
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
        if (confirm(`Are you sure you want to delete '${item.file_path}'?`)) {
          await delete_item(item.file_path);
          reload_page();
        }
        break;
      case "new_file":
        var new_path = [...$WindowTabs[activeTab].currentPath];
        var new_name = prompt("Enter name for new file:").trim();
        if (new_name) {
          new_path.push(new_name);
          await new_file(new_path.join("\\"));
          reload_page();
        }
        break;
      case "new_folder":
        var new_path = [...$WindowTabs[activeTab].currentPath];
        var new_name = prompt("Enter name for new folder:").trim();
        if (new_name) {
          new_path.push(new_name);
          await new_folder(new_path.join("\\"));
          reload_page();
        }
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
      case "refresh":
        reload_page();
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
  const show_window_context_options = (e, item) => {
    $ContextMenuList.items = [
      { label: "Refresh", callback: handleEvent, args: [e, item, "refresh"] },
      { label: "New File", callback: handleEvent, args: [e, item, "new_file"] },
      { label: "New Folder", callback: handleEvent, args: [e, item, "new_folder"] },
      { label: "Properties", callback: handleEvent, args: [e, { file_path: $WindowTabs[activeTab].currentPath.join("\\") }, "properties"] },
    ];
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
  let fields = [
    { label: "Name", field: "file_name", width: "w-full" },
    { label: "Date modified", field: "file_modification_time", width: "w-1/5" },
    { label: "Type", field: "file_ext", width: "w-1/6" },
    { label: "Size", field: "file_size", width: "w-1/6" },
  ];
</script>

<div
  id="explorer-view-page"
  class="flex flex-row h-full"
  on:contextmenu|preventDefault|stopPropagation={(e) => createContextMenu(e, show_window_context_options)}
>
  <!-- Tile View -->
  {#if $WindowTabs[activeTab].viewMode === 0}
    <ul
      id="file-list"
      class="flex flex-row flex-wrap px-2 h-min"
    >
      {#if $WindowTabs[activeTab].currentView.length === 0}
        <li class="text-primarytext">Empty</li>
      {/if}

      {#each $WindowTabs[activeTab].currentView as item, idx}
        <li
          class="flex items-center p-2 w-64 h-16 hover:bg-surfacebackground hover:cursor-pointer rounded-md overflow-hidden"
          on:dblclick|preventDefault={(e) => handleEvent(e, item, "open")}
          on:contextmenu|preventDefault|stopPropagation={(e) => createContextMenu(e, show_context_options, item)}
        >
          <button
            class="flex items-center"
            tabindex={idx}
          >
            <i
              class="flex justify-center items-center w-16 h-16 icon text-3xl"
              class:icon-file={!item.file_attributes.directory}
              class:icon-folder={item.file_attributes.directory}
            ></i>

            <ul class="flex flex-col text-xs">
              <span class="flex w-full text-nowrap">{item.file_name.length > 30 ? item.file_name.slice(0, 30) + "..." : item.file_name}</span>
              {#if !item.file_attributes.directory}
                <span class="flex">{item.file_ext.slice(0,5)} file</span>
                <span class="flex">{formatBytes(item.file_size)}</span>
              {/if}
            </ul>
          </button>
        </li>
      {/each}
    </ul>
    <!-- List View -->
  {:else if $WindowTabs[activeTab].viewMode === 1}
    <ul
      id="file-list"
      class="flex flex-col flex-wrap px-2 w-full h-min"
    >
      <!-- Header Row with Sorting Options -->
      <li class="flex items-center p-2 w-full">
        <button class="flex flex-row items-center w-full">
          {#each fields as { label, field, width }}
            <span
              class="flex px-2 py-1 overflow-hidden text-nowrap text-left cursor-pointer {width}"
              on:click={() => handle_sort(field)}
            >
              {label}
              <i
                class="icon text-xs p-1"
                class:icon-arrow-down-small-big={sortField === field && sortAsc}
                class:icon-arrow-up-big-small={sortField === field && !sortAsc}
              ></i>
            </span>
          {/each}
        </button>
      </li>

      {#if $WindowTabs[activeTab].currentView.length === 0}
        <li class="text-primarytext">Empty</li>
      {/if}

      {#each $WindowTabs[activeTab].currentView as item}
        <li class="flex items-center p-2 w-full overflow-hidden hover:bg-surfacebackground rounded-md">
          <button
            class="flex items-center w-full"
            on:dblclick|preventDefault={(e) => handleEvent(e, item, "open")}
            on:contextmenu|preventDefault|stopPropagation={(e) => createContextMenu(e, show_context_options, item)}
            tabindex="0"
          >
            <i class={`flex justify-center items-center w-8 h-8 icon icon-${item.file_attributes.directory ? "folder" : "file"} cursor-pointer text-lg`}></i>
            <ul class="flex flex-row text-sm w-full h-10 justify-between">
              <span class="flex px-2 py-1 w-full overflow-hidden text-wrap text-left">{item.file_name}</span>
              <span class="flex px-2 py-1 w-1/5 overflow-hidden text-wrap items-center">{formatDates(item.file_modification_time)}</span>
              <span class="flex px-2 w-2/12 overflow-hidden text-nowrap items-center">
                {#if !item.file_attributes.directory}
                  {item.file_ext.slice(0,5)} file
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
</div>

<style>
</style>
