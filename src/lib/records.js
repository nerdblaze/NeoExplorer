import { writable } from "svelte/store";

export const WindowTabs = writable([]);
export const Notification = writable({
    "body":"",
    "show":false,
    "type":"info",
});