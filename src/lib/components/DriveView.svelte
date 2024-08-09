<script>
  import { fetch_drives, open_folder } from "$lib/common";
  import { WindowTabs } from "$lib/records";
  import { formatBytes, get_active_tab } from "$lib/utilities";
  import { onDestroy, onMount } from "svelte";

  let drives = [];

  // Mount component: load drives and set up event listeners
  onMount(async () => {
    const activeTab = get_active_tab();
    if ($WindowTabs[activeTab].currentPath.length == 0) {
      drives = await fetch_drives();
    }
  });

</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<ul
  id="drive-list"
  class="flex flex-row p-2 flex-wrap"
>
  {#if drives.length === 0}
    <li class="text-primarytext">Nothing to show</li>
  {/if}
  {#each drives as drive}
    <li
      class="flex items-center p-4 bg-primarybackground hover:bg-secondarybackground transition-colors w-72 hover:cursor-pointer"
      on:dblclick={() => open_folder(`${drive.disk_label}:`)}
    >
      <i class="icon icon-hard-drive text-2xl text-accentprimary mr-4"></i>
      <div class="flex-1">
        <div class="text-lg font-semibold text-primarytext">{drive.disk_name} ({drive.disk_label}):</div>
        <div class="w-full h-2 bg-secondarybackground rounded-full mt-2">
          <div
            class="h-full bg-accentprimary rounded-full"
            style="width: {((drive.total_space - drive.free_space) / drive.total_space) * 100}%;"
          ></div>
        </div>
        <div class="text-sm text-hinttext mt-1">
          {formatBytes(drive.total_space - drive.free_space)} used of {formatBytes(drive.total_space)}
        </div>
      </div>
    </li>
  {/each}
</ul>

<style>
</style>
