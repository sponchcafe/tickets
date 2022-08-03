<script lang="ts">
    import TicketPreview from "./TicketPreview.svelte";
    let ticket_title: string = "New ticket";
    let ticket_body: string = "Ticket body";
</script>

<div class="container d-flex">
    <form class="p-2 col-6">
        <label for="title">Title</label>
        <div class="form-group">
            <input
                class="form-control"
                bind:value={ticket_title}
                type="text"
                id="title"
                name="title"
            />
        </div>
        <div class="form-group">
            <label for="body">Content</label>
            <textarea
                class="form-control"
                bind:value={ticket_body}
                id="body"
                name="body"
            />
        </div>
        <div class="form-group">
            <input
                type="submit"
                value="Create"
                class="btn m-2 ml-auto btn-primary"
            />
        </div>
    </form>
    <div class="col-6">
        <TicketPreview {ticket_body} {ticket_title} />
    </div>
</div>
