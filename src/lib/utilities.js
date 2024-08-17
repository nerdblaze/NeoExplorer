import { Notification, WindowTabs, StatusInfo } from "$lib/records.js";
import dayjs from "dayjs";

export const formatBytes = (bytes) => {
  if (bytes === 0) return "0 B";

  const units = ["B", "KB", "MB", "GB", "TB", "PB"];
  const k = 1024;
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${(bytes / Math.pow(k, i)).toFixed(2)} ${units[i]}`;
};

export const formatDates = (seconds) => {
  return dayjs(seconds * 1000).format("DD/MM/YYYY HH:mm");
};

export const notify = async (msg, typ = "Info") => {
  // Define the valid types
  const validTypes = ["Primary", "Secondary", "Error", "Warning", "Info", "Success"];

  // Set type to 'info' if it's not valid
  const type = validTypes.includes(typ) ? typ : "Info";

  // Show the notification
  Notification.update((current) => {
    return {
      ...current,
      body: msg,
      show: true,
      type: type,
      old: [...current.old, { type: type, msg: msg }],
    };
  });

  // Hide the notification after 1 second
  setTimeout(() => {
    Notification.update((current) => {
      return {
        ...current,
        show: false,
      };
    });
  }, 1000);
};

export const clear_notification = async (index) => {
  Notification.update((current) => {
    const updatedItems = index || index === 0 ? [...current.old.slice(0, index), ...current.old.slice(index + 1)] : [];
    return { ...current, old: [...updatedItems] };
  });
};

export const get_active_tab = () => {
  let index = -1;
  WindowTabs.subscribe((items) => {
    index = items.findIndex((item) => item.isActive);
  })();
  return index;
};
export const update_file_count = async (file_count) => {
  StatusInfo.update((status) => {
    return {
      ...status,
      file_count,
    };
  });
};
