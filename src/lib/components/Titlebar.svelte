<script>
  import { new_window } from "$lib/common.js";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { getAllWebviewWindows } from "@tauri-apps/api/webviewWindow";

  export let title = "";
  let isWindowed = true;

  const minimize_window = async (event) => {
    const appWindow = getCurrentWindow();
    appWindow.minimize();
  };

  const maximize_window = async (event) => {
    const appWindow = getCurrentWindow();
    appWindow.toggleMaximize();
    isWindowed = await appWindow.isMaximized();
  };

  const close_window = async (event) => {
    const appWindow = getCurrentWindow();
    let windows = getAllWebviewWindows();
    if (windows.length > 1) {
      appWindow.close();
    } else {
      appWindow.hide();
    }
  };

  let isWindowDragging = false;
  let windowPosition = { x: 0, y: 0, isLive: false };

  const startMove = async (event) => {
    const titleBarMover = document.querySelector("#window-mover");
    isWindowDragging = titleBarMover === event.target;

    const dragWindow = async (e) => {
      if (isWindowDragging) {
        if (windowPosition.isLive === false) {
          windowPosition = { x: event.clientX, y: event.clientY, isLive: true };
        }
        const curPosition = await getCurrentWindow().outerPosition();
        curPosition.x = curPosition.x - (windowPosition.x - e.clientX);
        curPosition.y = curPosition.y - (windowPosition.y - e.clientY);
        await getCurrentWindow().setPosition(curPosition);
      }
    };

    const stopMove = () => {
      isWindowDragging = false;
      windowPosition.isLive = false;
      window.removeEventListener("mousemove", dragWindow);
      window.removeEventListener("mouseup", stopMove);
    };

    window.addEventListener("mousemove", dragWindow);
    window.addEventListener("mouseup", stopMove);
  };
</script>

<header class="flex flex-row justify-end h-8 w-full bg-secondarybackground">
  <div
    id="window-mover"
    class="cursor-move flex grow min-w-16"
    on:mousedown={startMove}
    role="button"
    tabindex=""
  ><span class="flex px-2 text-xl font-semibold">{title.slice(0,48)}</span></div>
  <div
    id="window-button-container"
    class="hidden sm:flex flex-row h-8 text-sm text-center"
  >
    <a
      class="flex relative items-center justify-center w-8 hover:bg-surfacebackground cursor-pointer"
      href="/"
      on:click|preventDefault={() => new_window("/","")}
    >
      <i class="absolute icon icon-window text-xs"></i>
      <i class="absolute right-1 bottom-1 icon icon-circle-plus bg-secondarybackground text-2xs"></i>
    </a>
    <a
      class="flex relative items-center justify-center w-8 hover:bg-surfacebackground cursor-pointer"
      href="/"
      on:click|preventDefault={() => minimize_window()}
    >
      <i class="icon icon-minus text-xs"></i>
    </a>
    <a
      class="flex relative items-center justify-center w-8 hover:bg-surfacebackground cursor-pointer"
      href="/"
      on:click|preventDefault={() => maximize_window()}
    >
      <i class="icon icon-{isWindowed ? 'expand' : 'compress'} text-xs"></i>
    </a>
    <a
      class="flex relative items-center justify-center w-8 hover:bg-accenterror cursor-pointer"
      href="/"
      on:click|preventDefault={() => close_window()}
    >
      <i class="icon icon-x text-xs"></i>
    </a>
  </div>
</header>

<style>
</style>
