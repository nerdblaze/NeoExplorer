import { Notification } from "./records.js";

export function formatBytes(bytes) {
  if (bytes === 0) return "0 B";

  const units = ["B", "KB", "MB", "GB", "TB", "PB"];
  const k = 1024;
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${(bytes / Math.pow(k, i)).toFixed(2)} ${units[i]}`;
}

export function notify(msg, typ = 'info') {
    // Define the valid types
    const validTypes = ['primary', 'secondary', 'error', 'warning', 'info', 'success'];
    
    // Set type to 'info' if it's not valid
    const type = validTypes.includes(typ) ? typ : 'info';
  
    // Show the notification
    Notification.set({
      body: msg,
      show: true,
      type: type
    });
  
    // Hide the notification after 1 second
    setTimeout(() => {
      Notification.set({
        body: "",
        show: false,
        type: "info" // Reset to default type
      });
    }, 1000);
  }