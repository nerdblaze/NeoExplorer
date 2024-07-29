<script>
  import { formatBytes } from "$lib/utilities";
  import { createEventDispatcher } from "svelte";

  export let drive = {};
  const dispatch = createEventDispatcher();

  function open_drive() {
    dispatch('click', { });
  }
</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<li class="flex items-center p-4 bg-primarybackground hover:bg-secondarybackground transition-colors w-72 hover:cursor-pointer" on:click={open_drive}>
  <i class="icon icon-hard-drive text-2xl text-accentprimary mr-4"></i>
  <div class="flex-1">
    <div class="text-lg font-semibold text-primarytext">{drive.drive_name} ({drive.drive_label}):</div>
    <div class="w-full h-2 bg-secondarybackground rounded-full mt-2">
      <div
        class="h-full bg-accentprimary rounded-full"
        style="width: {(drive.used_space / drive.total_space) * 100}%;"
      ></div>
    </div>
    <div class="text-sm text-hinttext mt-1">
      {formatBytes(drive.used_space)} used of {formatBytes(drive.total_space)}
    </div>
  </div>
</li>

<style>
</style>

