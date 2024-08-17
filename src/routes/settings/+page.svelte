<script>
  import WidgetSelection from "$lib/widgets/WidgetSelection.svelte";
  import WidgetColorPicker from "$lib/widgets/WidgetColorPicker.svelte";

  let settings_page = [
    {
      category: "General",
      layout: "flex-col",
      icon: "gear",
      widgets: [
        {
          type: WidgetSelection,
          params: {
            label: "Language",
            options: [
              { label: "English", value: 0 },
              { label: "Chinese", value: 1 },
              { label: "Hindi", value: 2 },
              { label: "Spanish", value: 3 },
              { label: "French", value: 4 },
              { label: "Arabic", value: 5 },
              { label: "Bengali", value: 6 },
            ],
          },
          value: 0,
        },
      ],
      active: true,
    },
    {
      category: "UI",
      layout: "flex-inline",
      icon: "paintbrush-pencil",
      widgets: [
        { type: WidgetColorPicker, params: { label: "Primary Background" }, value: "#ffffff" },
        { type: WidgetColorPicker, params: { label: "Secondary Background" }, value: "#ffffff" },
        { type: WidgetColorPicker, params: { label: "Surface Background" }, value: "#ffffff" },
        { type: WidgetColorPicker, params: { label: "Primary Text" }, value: "#ffffff" },
        { type: WidgetColorPicker, params: { label: "Secondary Text" }, value: "#ffffff" },
        { type: WidgetColorPicker, params: { label: "Hint Text" }, value: "#ffffff" },
        { type: WidgetColorPicker, params: { label: "Accent Primary" }, value: "#ffffff" },
        { type: WidgetColorPicker, params: { label: "Accent Secondary" }, value: "#ffffff" },
        { type: WidgetColorPicker, params: { label: "Accent Error" }, value: "#ffffff" },
        { type: WidgetColorPicker, params: { label: "Accent Warning" }, value: "#ffffff" },
        { type: WidgetColorPicker, params: { label: "Accent Info" }, value: "#ffffff" },
        { type: WidgetColorPicker, params: { label: "Accent Success" }, value: "#ffffff" },
        { type: WidgetColorPicker, params: { label: "Divider Line" }, value: "#ffffff" },
      ],
      active: false,
    },
    { category: "Apps", icon: "browser", widgets: [], active: false },
    { category: "Security", icon: "shield-quartered", widgets: [], active: false },
    { category: "Tags", icon: "tags", widgets: [], active: false },
  ];
  const switch_settings = (index) => {
    settings_page = settings_page.map((item, idx) => ({
      ...item,
      active: index === idx,
    }));
  };
</script>

<aside
  id="settings-sidebar-container"
  class="m-2 p-2 border-r border-dividerline"
>
  <div class="flex flex-col sticky top-0">
    {#each settings_page as page, idx}
      <button
        class="text-lg text-left p-2 rounded-md hover:bg-surfacebackground"
        on:click|preventDefault={() => switch_settings(idx)}
      >
        <i class="icon icon-{page.icon} text-xs"></i>
        {page.category}
      </button>
    {/each}
  </div>
</aside>

<div
  id="settings-page-container"
  class="flex overflow-y-scroll"
>
  <div class="">
    {#each settings_page as page}
      {#if page.active}
        {#each page.widgets as widget}
          <svelte:component
            this={widget.type}
            {...widget.params}
            bind:value={widget.value}
          />
        {/each}
      {/if}
    {/each}
  </div>
</div>

<style>
  #settings-page-container {
    scrollbar-width: none;
  }
</style>
