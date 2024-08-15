<script>
  export let value = "#ffffff";
  export let label = "";
</script>

<div class="flex items-center space-x-2">
  <!-- Label for the color picker -->
  <span class="font-medium">{label}</span>
  
  <!-- Color box and input -->
  <div class="relative">
    <div
      class="rounded-md border h-8 w-8"
      style="background-color: {value};"
    ></div>
    <input
      type="color"
      class="absolute top-0 left-0 opacity-0 w-full h-full cursor-pointer"
      bind:value="{value}"
    />
  </div>
</div>
