import { invoke } from "@tauri-apps/api/core";
import { once } from "@tauri-apps/api/event";
import { ContextMenuList, WindowTabs } from "$lib/records";
import { notify, get_active_tab, update_file_count } from "$lib/utilities";
import TabWindowComponent from "$lib/components/TabWindowComponent.svelte";

// Function to hide the context menu
export const hideContextMenu = async (event) => {
  const contextMenu = document.querySelector("#context-menu-container");
  if (event.target == contextMenu) return;
  ContextMenuList.update((current) => {
    return {
      ...current,
      visible: false,
    };
  });
};

export const showContextMenu = async (event) => {
  ContextMenuList.update((menu) => {
    return {
      ...menu,
      visible: true,
    };
  });
};

// Main function to determine which context menu to show based on item type
export const createContextMenu = async (event, callback, ...args) => {
  showContextMenu(event);
  await callback(event, ...args);

  const contextMenu = document.querySelector("#context-menu-container");
  const ww = window.innerWidth;
  const wh = window.innerHeight;
  const cw = contextMenu.clientWidth;
  const ch = contextMenu.clientHeight;
  const posx = cw + event.clientX >= ww ? ww - cw - 8 : event.clientX;
  const posy = ch + event.clientY >= wh ? wh - ch - 8 : event.clientY;

  ContextMenuList.update((menu) => {
    return {
      ...menu,
      x: posx,
      y: posy,
    };
  });
};

const process_response = async (item) => {
  await new Promise((resolve) => setTimeout(resolve, 100)); // Simulate async delay
  let file_name = item.file_path.split("\\").pop();
  let file_ext = file_name.slice(file_name.lastIndexOf(".") + 1);
  return { ...item, file_name, file_ext };
};

// Open a folder and handle potential errors
export const open_folder = async (folderPath) => {
  const activeTab = get_active_tab();
  const pathBuff = folderPath.split("\\").filter(Boolean);
  try {
    if (folderPath !== "") {
      folderPath = pathBuff.join("\\") + "\\";
      let response = await invoke("open_folder", { folderPath });
      response = await Promise.all(response.map(process_response));

      update_file_count(response.length);

      WindowTabs.update((items) => {
        items[activeTab] = {
          ...items[activeTab],
          currentPath: [...pathBuff],
          currentView: [...response],
        };
        return items;
      });
    } else {
      WindowTabs.update((items) => {
        items[activeTab] = {
          ...items[activeTab],
          currentPath: [...[]],
          currentView: [...[]],
        };
        return items;
      });
    }
  } catch (error) {
    notify(error, "Error");
  }
};
export const open_file = async (filePath) => {
  try {
    if (filePath !== "") {
      const response = await invoke("open_file", { filePath });
    } else {
      notify("No files selected", "Info");
    }
  } catch (error) {
    notify(error, "Error");
  }
};
export const fetch_drives = async () => {
  let response = await invoke("list_drives", {});
  update_file_count(response.length);
  return response;
};
export const search_system = async (search_term) => {
  let search_results = [];
  const activeTab = get_active_tab();
  const limit = 10000;
  const params = { path: search_term, limit };
  search_results = await invoke("search_system", { params });
  search_results = await Promise.all(search_results.map(process_response));
  update_file_count(search_results.length);
  WindowTabs.update((items) => {
    items[activeTab] = {
      ...items[activeTab],
      currentView: [...search_results],
    };
    return items;
  });
};

export const new_window = async (initUrl = "", folderPath = "") => {
  return await invoke("create_new_window", { initUrl, folderPath });
};

export const init_window = async (handler) => {
  let unlisten = await once("initialize", handler);
  setTimeout(() => {
    if (unlisten) {
      unlisten();
    }
  }, 500);
};

// Function to add a new tab
export const new_tab = async (file_path) => {
  WindowTabs.update((tabs) => {
    return [
      ...tabs,
      {
        component: TabWindowComponent,
        isActive: false,
        viewMode: 0,
        hasPreview: false,
        searchTerm: "",
        currentPath: [],
        oldPath: [],
        currentView: [],
      },
    ];
  });
  await switch_tab(-1);
  if (file_path) open_folder(file_path);
};

// Function to activate a tab
export const switch_tab = async (index) => {
  WindowTabs.update((items) => {
    index = index == -1 ? items.length - 1 : index;
    update_file_count(items[index].currentView.length);
    return items.map((tab, idx) => ({
      ...tab,
      isActive: idx == index,
    }));
  });
};

export const get_properties = async (filePath) => {
  return await invoke("get_file_info", { filePath });
};

// Reload the current folder
export const reload_page = async () => {
  const activeTab = get_active_tab();
  const unsub = WindowTabs.subscribe(async (item) => {
    if (item[activeTab].currentPath.length > 0) {
      let path = item[activeTab].currentPath.join("\\");
      await open_folder(path);
    }
  });
  unsub();
};

export const delete_item = async (filePath) => {
  try {
    return await invoke("delete_file", { filePath });
  } catch (error) {
    notify(error, "Error");
  }
};

export const delete_items = async (filePaths) => {
  try {
    return await invoke("delete_files", { filePaths });
  } catch (error) {
    notify(error, "Error");
  }
};

export const new_file = async (filePath) => {
  try {
    return await invoke("create_file", { filePath });
  } catch (error) {
    notify(error, "Error");
  }
};
export const new_folder = async (folderPath) => {
  try {
    return await invoke("create_folder", { folderPath });
  } catch (error) {
    notify(error, "Error");
  }
};
