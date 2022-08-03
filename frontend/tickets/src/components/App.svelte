<script lang="ts">
  import NavBar from "./NavBar.svelte";
  import SideBar from "./SideBar.svelte";
  import CreateTicket from "./CreateTicket.svelte";
  import TicketList from "./TicketList.svelte";
</script>

<main>
  <NavBar />
  <div class="container m-0">
    <div class="row">
      <SideBar />
      <div class="col-10 flex-column p-4 flex-wrap bd-highlight">
        <h3 class="mb-4 border-bottom">Create Ticket</h3>
        <CreateTicket />
        <h3 class="mb-4 border-bottom">Backlog</h3>
        <TicketList />
      </div>
    </div>
  </div>
</main>
