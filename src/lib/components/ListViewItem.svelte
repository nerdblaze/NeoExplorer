<script>
  import { createEventDispatcher } from "svelte";

    export let item;
  
    const dispatch = createEventDispatcher();

    function open_item() {
        dispatch('click', { });
    }

  </script>
  
  <li class="search-result-item flex items-center p-2" >
    <a href="/" on:dblclick={open_item} role="button"><i class="icon icon-{item.file_attributes.directory ? 'folder' : 'file'} cursor-pointer"></i>{item.file_name}</a>    
  </li>
  
  <style>
  </style>
  