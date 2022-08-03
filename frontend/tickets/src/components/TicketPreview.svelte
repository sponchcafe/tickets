<script lang="ts">
    export let ticket_title: string;
    export let ticket_body: string;
    import { marked } from "marked";
</script>

<div class="p-4 bg-light border rounded">
    <h4>{ticket_title}</h4>
    <p>{@html marked(ticket_body)}</p>
</div>
