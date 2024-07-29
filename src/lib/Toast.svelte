<script>
    import { Notification } from "./records";
</script>

{#if $Notification.show}
<div class="fixed top-0 right-0 mb-4 mr-4 z-20" id="toast-notification">
    <!-- Success Toast -->
    <div class="bg-accent{$Notification.type} text-text p-4 mb-2 rounded shadow-lg">
      <p class="font-semibold">{$Notification.type}!</p>
      <p class="text-sm">{$Notification.body}</p>
    </div>
  
</div>
{/if}