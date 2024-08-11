<script>
  import { WindowTabs, ContextMenuList } from "$lib/records.js";
  import { createContextMenu, open_folder, new_window, new_tab } from "$lib/common.js";
  import { get_active_tab, notify } from "$lib/utilities";

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
    open_folder(path);
  };
  const show_properties  = async (e, item) => {
    notify("#TODO[Dummy ContextMenu]", "Info");
  }
  const delete_item  = async (e, item) => {
    notify("#TODO[Dummy ContextMenu]", "Warning");
  }
  const open_in_tab  = async (e, item) => {
    new_tab(item.file_path);
  }
  const open_in_window  = async (e, item) => {
    new_window(item.file_path);
  }
    const show_context_options = async (e, item) => {
    $ContextMenuList.items = [...[
      {label :"Open", callback : open_item, args : [e,item]},
      {label :"Open in new tab", callback : open_in_tab, args : [e,item]},
      {label :"Open in new window", callback : open_in_window, args : [e,item]},
      {label :"Delete", callback : delete_item, args : [e,item]},
      {label :"Properties", callback : show_properties, args : [e,item]},
    ]]
  }
</script>

<ul
  id="file-list"
  class="flex flex-row flex-wrap p-2"
>
  {#if $WindowTabs[activeTab].currentView.length === 0}
    <li class="text-primarytext">Empty</li>
  {/if}

  {#each $WindowTabs[activeTab].currentView as item}
    <li class="search-result-item flex items-center p-2">
      <a
        href="/"
        on:dblclick|preventDefault={(e) => open_item(e, item)}
        on:contextmenu|preventDefault={(e) => createContextMenu(e, show_context_options, e, item)}
        role="button"
      >
        <i class="icon icon-{item.file_attributes.directory ? 'folder' : 'file'} cursor-pointer"></i>
        {item.file_name}
      </a>
    </li>
  {/each}
</ul>

<style>
</style>
