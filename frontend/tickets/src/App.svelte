<script lang="ts">
  import { onMount } from "svelte";
  import get_all_tickets from "./lib/api.js";
  import Ticket from "./lib/Ticket.svelte";
  import type {TicketData} from "./lib/api.js"
  let tickets: TicketData[] = [];
  onMount(async () => (tickets = await get_all_tickets()));
</script>

<main>
  <div>
    {#each tickets as { title, description }, i}
      <Ticket {title} {description} />
    {/each}
  </div>
</main>

<style>
  div {
    display: flex;
    flex-wrap: wrap;
  }
</style>
