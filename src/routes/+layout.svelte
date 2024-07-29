<script>
    import "../app.css";
    import "../icon.css"
    import Toast from "$lib/Toast.svelte";
  import { StatusInfo } from "$lib/records";
</script>

<div class="w-full h-dvh flex flex-col">
    <slot />
    <!-- Status bar -->
    <div id="status-bar" class="w-full h-8 sticky z-50 bottom-0 flex flex-row justify-between p-2 text-sm border-t border-secondarybackground bg-primarybackground">
        <ul id="file-info" class="flex flex-row">
            <li id="file-info-count" class="mx-2">{$StatusInfo.file_count} item</li>
            <li id="file-info-size" class="mx-2">784 bytes</li>
            <li id="file-info-permission" class="mx-2">777</li>
            <li id="file-info-encryption" class="mx-2"><i class="icon icon-file-lock text-2xs"></i> none</li>
        </ul>
        <ul id="explorer-info" class="flex flex-row">
            <li id="explorer-info-git" class="mx-2"><i class="icon icon-code-branch text-2xs py-0.5"></i></li>
            <li id="explorer-info-theme" class="mx-2"><i class="icon icon-sun text-2xs"></i></li>
            <li id="explorer-info-notification" class="mx-2"><i class="icon icon-bell text-2xs"></i></li>
        </ul>
    </div>
    <Toast/>
</div>

