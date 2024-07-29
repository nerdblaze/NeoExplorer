<script>
  import { createEventDispatcher } from "svelte";

    export let item;
  
    const itemClass = item.file_attributes.directory ? 'folder' : 'file';
  
    const dispatch = createEventDispatcher();

    function open_item() {
        dispatch('click', { });
    }

  </script>
  
  <li class="search-result-item flex items-center p-2 cursor-pointer" on:click={open_item}>
    <i class="icon icon-{itemClass}" role="button"></i>
    {item.file_name}
  </li>
  
  <style>
    .search-result-item {
      transition: background-color 0.2s ease;
    }
    .search-result-item:hover {
      background-color: #f0f0f0;
    }
  </style>
  