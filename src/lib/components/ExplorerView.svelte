<script>
  import { WindowTabs, ContextMenuList } from "$lib/records.js";
  import { createContextMenu, open_folder, open_file, new_window, new_tab } from "$lib/common.js";
  import { get_active_tab, notify, formatBytes, formatDates } from "$lib/utilities";

  const activeTab = get_active_tab();

  const open_item = async (e, item) => {
    let path = "";
    if ($WindowTabs[activeTab].searchTerm) {
      path = item.file_path;
    } else {
      let pathbuff = [...$WindowTabs[activeTab].currentPath];
      pathbuff.push(item.file_name);
      path = pathbuff.join("\\");
    }
    if (item.file_attributes.directory) {
      open_folder(path);
    } else {
      open_file(path);
    }
  };
  const show_properties = async (e, item) => {
    notify("#TODO[Dummy ContextMenu]", "Info");
  };
  const delete_item = async (e, item) => {
    notify("#TODO[Dummy ContextMenu]", "Warning");
  };
  const open_in_tab = async (e, item) => {
    if (item.file_attributes.directory) {
      new_tab(item.file_path);
    }
  };
  const open_in_window = async (e, item) => {
    if (item.file_attributes.directory) {
      new_window(item.file_path);
    }
  };
  const show_context_options = async (e, item) => {
    $ContextMenuList.items = [
      ...[
        { label: "Open", callback: open_item, args: [e, item] },
        { label: "Open in new tab", callback: open_in_tab, args: [e, item] },
        { label: "Open in new window", callback: open_in_window, args: [e, item] },
        { label: "Delete", callback: delete_item, args: [e, item] },
        { label: "Properties", callback: show_properties, args: [e, item] },
      ],
    ];
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

    {#each $WindowTabs[activeTab].currentView as item}
      <li class="flex items-center p-2 min-w-64 w-full sm:w-1/2 md:w-1/4 lg:w-1/6">
        <a
          class="flex items-center"
          href="/"
          on:dblclick|preventDefault={(e) => open_item(e, item)}
          on:contextmenu|preventDefault={(e) => createContextMenu(e, show_context_options, e, item)}
          role="button"
        >
          <i class="flex justify-center items-center w-16 h-16 icon icon-{item.file_attributes.directory ? 'folder' : 'file'} cursor-pointer text-3xl"></i>
          <ul class="flex flex-col text-xs">
            <span class="flex">{item.file_name.length > 31 ? item.file_name.slice(0, 31) + "..." : item.file_name}</span>
            {#if !item.file_attributes.directory}
              <span class="flex">{item.file_name.slice(item.file_name.lastIndexOf(".") + 1)} file</span>
              <span class="flex">{formatBytes(item.file_size)}</span>
            {/if}
          </ul>
        </a>
      </li>
    {/each}
  </ul>
  <!-- List View -->
{:else if $WindowTabs[activeTab].viewMode == 1}
  <ul
    id="file-list"
    class="flex flex-col flex-wrap p-2 w-full"
  >
    {#if $WindowTabs[activeTab].currentView.length === 0}
      <li class="text-primarytext">Empty</li>
    {/if}

    {#each $WindowTabs[activeTab].currentView as item}
      <li class="flex items-center p-2 w-full overflow-hidden">
        <a
          class="flex items-center w-full"
          href="/"
          on:dblclick|preventDefault={(e) => open_item(e, item)}
          on:contextmenu|preventDefault={(e) => createContextMenu(e, show_context_options, e, item)}
          role="button"
        >
          <i class="flex justify-center items-center w-8 h-8 icon icon-{item.file_attributes.directory ? 'folder' : 'file'} cursor-pointer text-lg"></i>
          <ul class="flex flex-row text-sm w-full h-10 justify-between">
            <span class="flex px-2 py-1 w-full overflow-hidden text-wrap">{item.file_name}</span>
            <span class="flex px-2 py-1 w-1/5 overflow-hidden text-wrap items-center">{formatDates(item.file_modification_time)}</span>
            <span class="flex px-2 w-2/12 overflow-hidden text-wrap items-center">
              {#if !item.file_attributes.directory}
                {item.file_name.slice(item.file_name.lastIndexOf(".") + 1)} file
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
        </a>
      </li>
    {/each}
  </ul>
{/if}

<style>
</style>
