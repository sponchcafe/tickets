<script lang="ts">
    import Ticket from "./Ticket.svelte";
    import { onMount } from "svelte";
    import get_all_tickets from "../lib/api";
    import type { TicketData } from "../lib/api";
    let tickets: TicketData[] = [];
    onMount(async () => (tickets = await get_all_tickets()));
</script>

<table class="table table-striped">
    <thead class="thead-light">
        <tr>
            <th scope="col"> ID </th>
            <th scope="col"> Description </th>
        </tr>
    </thead>
    <tbody>
        {#each tickets as { title, description }, i}
            <Ticket {title} {description} />
        {/each}
    </tbody>
</table>
