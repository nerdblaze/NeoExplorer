import { writable, derived } from "svelte/store";

export const WindowTabs = writable([]);
export const Notification = writable({
    body:"",
    show:false,
    type:"info",
});
export const StatusInfo = writable({
    file_count: 0,
});
export const ContextMenuList = writable({
    visible: false,
    x: 0,
    y: 0,
    items: [] 
});