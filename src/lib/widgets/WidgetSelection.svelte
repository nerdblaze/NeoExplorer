<script>
  export let options = [];
  export let value;
  export let label = "";
  let rand_id = "";
</script>

<div class="flex flex-row">
  {#if label}
    <label class="flex pr-2 text-md items-center" for="">{label}</label>
  {/if}
  <select
    class="bg-primarybackground border border-dividerline rounded-md p-2 w-full text-sm focus:ring-2 focus:ring-accentprimary"
    bind:value
  >
    {#each options as option}
      <option
        value={option.value}
        class="hover:bg-surfacebackground"
      >
        {option.label}
      </option>
    {/each}
  </select>
</div>

<style></style>
