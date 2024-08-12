import { ContextMenuList, WindowTabs } from "./records";
import { notify, get_active_tab, update_file_count } from "./utilities";
import { invoke } from "@tauri-apps/api/core";
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
  await callback(...args);

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

// Open a folder and handle potential errors
export const open_folder = async (folderPath) => {
  const activeTab = get_active_tab();
  const pathBuff = folderPath.split("\\").filter(Boolean);
  try {
    if (folderPath !== "") {
      folderPath = pathBuff.join("\\") + "\\";
      const response = await invoke("open_folder", { folderPath });
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
  let page = 1;
  const activeTab = get_active_tab();
  const pageSize = 10000;
  search_results = await invoke("search_system", { searchTerm: search_term, page, pageSize });
  update_file_count(search_results.length);
  WindowTabs.update((items) => {
    items[activeTab] = {
      ...items[activeTab],
      currentView: [...search_results],
    };
    return items;
  });
};

export const new_window = async (folderPath) => {
  const response = await invoke("create_new_window", { folderPath });
};

// Function to add a new tab
export const new_tab = async (file_path) => {
  WindowTabs.update((tabs) => {
    return [
      ...tabs,
      {
        component: TabWindowComponent,
        isActive: false,
        hasPreview: false,
        searchTerm: "",
        currentPath: [],
        oldPath: [],
        currentView: [],
      },
    ];
  });
  switch_tab(-1);
  if (file_path) open_folder(file_path);
};

// Function to activate a tab
export const switch_tab = async (index) => {
  WindowTabs.update((items) => {
    return items.map((tab, idx) => ({
      ...tab,
      isActive: idx == (index == -1 ? items.length - 1 : index),
    }));
  });
};
