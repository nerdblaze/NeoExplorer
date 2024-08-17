<script>
  import { new_window } from "$lib/common";
  import { StatusInfo, Notification } from "$lib/records";
  import { clear_notification } from "$lib/utilities";
  import { onMount } from "svelte";

  let isDarkTheme = false;
  let isNotificationShown = false;
  const open_home = async () => {
    window.location.href = "/";
  };
  const open_settings = async () => {
    new_window("/settings","");
  };
  const toggle_theme = async () => {
    document.documentElement.setAttribute("data-theme", isDarkTheme ? "light" : "dark");
    isDarkTheme = !isDarkTheme;
  };
  const toggle_notification = async () => {
    isNotificationShown = !isNotificationShown;
  };
  onMount(async () => {
    isDarkTheme = window.matchMedia && window.matchMedia("(prefers-color-scheme:dark)").matches;
    if (isDarkTheme) document.documentElement.setAttribute("data-theme", "dark");
  });
</script>

<!-- Status bar -->
<div
  id="status-bar"
  class="w-full h-8 sticky z-50 bottom-0 flex flex-row justify-between p-2 text-sm border-t border-surfacebackground bg-primarybackground"
>
  <ul
    id="file-info"
    class="flex flex-row"
  >
    <li
      id="file-info-count"
      class="mx-2"
    >
      {$StatusInfo.file_count}
      {$StatusInfo.file_count < 2 ? "item" : "items"}
    </li>
    <li
      id="file-info-size"
      class="mx-2"
    >
      784 bytes
    </li>
    <li
      id="file-info-permission"
      class="mx-2"
    >
      777
    </li>
    <li
      id="file-info-encryption"
      class="mx-2"
    >
      <i class="icon icon-file-lock text-2xs"></i>
      none
    </li>
  </ul>
  <span
    id="explorer-info"
    class="flex flex-row"
  >
    <button
      id="explorer-info-home"
      class="flex mx-2 text-center w-4"
      on:click|preventDefault={open_home}
    >
      <i class="icon icon-house text-2xs py-0.5 mx-auto"></i>
    </button>
    <button
      id="explorer-info-theme"
      class="flex mx-2 text-center w-4"
      on:click|preventDefault={toggle_theme}
    >
      <i class="icon icon-{isDarkTheme ? 'moon' : 'sun'} text-2xs py-0.5 mx-auto"></i>
    </button>
    <button
      id="explorer-info-settings"
      class="flex mx-2 text-center w-4"
      on:click|preventDefault={open_settings}
    >
      <i class="icon icon-gear text-2xs py-0.5 mx-auto"></i>
    </button>
    <button
      id="explorer-info-notification"
      class="flex mx-2 text-center w-4"
      on:click|preventDefault={toggle_notification}
    >
      {#if $Notification.old.length > 0}
        <i class="icon icon-bell-on text-2xs py-0.5 mx-auto"></i>
      {:else}
        <i class="icon icon-bell text-2xs py-0.5 mx-auto"></i>
      {/if}
    </button>
  </span>
  {#if isNotificationShown}
    <div
      id="notification-container"
      class="flex flex-col absolute bottom-8 right-0 max-h-32 min-w-32 max-w-full rounded-md shadow-md shadow-hinttext bg-primarybackground overflow-hidden m-2 border border-dividerline"
    >
      <div
        id="notification-header"
        class="flex justify-between w-full min-w-64 bg-secondarybackground p-1 border-b border-dividerline shadow-md"
      >
        <span class="font-bold">NOTIFICATIONS</span>
        <span>
          <button
            class=""
            on:click|preventDefault={() => clear_notification()}
          >
            <i class="icon icon-trash-list text-xs"></i>
          </button>
        </span>
      </div>
      <ul
        id="notification-contents"
        class="flex flex-col overflow-x-hidden overflow-y-auto"
        style="scrollbar-width: none;"
      >
        {#if $Notification.old.length > 0}
          {#each $Notification.old as notice, idx}
            <li
              class="flex p-0.5 border-b hover:opacity-80"
              style="background-color: rgb(from var(--Accent{notice.type}) r g b / 0.8); border-color: var(--Accent{notice.type});"
            >
              <span class="font-semibold">
                {notice.type}:
                <span class="font-normal pl-3 flex-wrap break-all">
                  {notice.msg}
                </span>
              </span>

              <button
                class="flex items-center"
                on:click|preventDefault={() => clear_notification(idx)}
              >
                <i class="icon icon-x text-2xs px-1"></i>
              </button>
            </li>
          {/each}
        {:else}
          <li class="flex p-0.5">No new notifications</li>
        {/if}
      </ul>
    </div>
  {/if}
</div>
