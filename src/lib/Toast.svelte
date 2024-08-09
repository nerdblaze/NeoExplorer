<script>
  import { Notification } from "./records";
</script>

{#if $Notification.show}
  <div
    class="fixed top-0 right-0 mb-4 mr-4 z-20"
    id="toast-notification"
  >
    <div
      class="text-primarytext p-4 mb-2 rounded shadow-lg ease-linear"
      style="background-color: var(--Accent{$Notification.type});"
    >
      <p class="font-semibold">{$Notification.type}!</p>
      <p class="text-sm">{$Notification.body}</p>
    </div>
  </div>
{/if}
