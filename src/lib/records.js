import { writable } from "svelte/store";

export const WindowTabs = writable([]);
export const Notification = writable({
    body:"",
    show:false,
    type:"info",
});
export const StatusInfo = writable({
    file_count: 0,
});