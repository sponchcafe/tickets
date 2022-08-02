<script lang="ts">
    import { marked } from "marked";
    let ticket_title: string = "New ticket";
    let ticket_body: string = "Ticket body";
</script>


<div class="container d-flex">

<form class="p-2 col-6">
    <div class="form-group">
        <label for="title">Title</label>
        <input class="form-control" bind:value={ticket_title} type="text" id="title" name="title" />
    </div>
    <div class="form-group">
        <label for="body">Content</label>
        <textarea class="form-control" bind:value={ticket_body} id="body" name="body" />
    </div>
    <input type="submit" value="Create" class="btn btn-primary" />
</form>
<div class="card col-6 p-4">
    <h4>{ticket_title}</h4>
    <p>{@html marked(ticket_body)}</p>
</div>

</div>