<script lang="ts">
  import get_all_tickets from "./lib/api.js";
  import Ticket from "./lib/Ticket.svelte";
  let tickets = [];
  get_all_tickets().then((t) => tickets = t)
</script>

<main>
  <div>
    {#each tickets as { title, description }, i}
      <Ticket {title} {description}></Ticket>
	  {/each}
  </div>
</main>

<style>
</style>
