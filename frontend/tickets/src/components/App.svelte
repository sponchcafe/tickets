<script lang="ts">
  import { onMount } from "svelte";
  import get_all_tickets from "../lib/api";
  import Ticket from "./Ticket.svelte";
  import type { TicketData } from "../lib/api";
  let tickets: TicketData[] = [];
  onMount(async () => (tickets = await get_all_tickets()));
</script>

<main>
  <nav class="navbar navbar-expand-lg navbar-dark bg-dark p-3">
    <a class="navbar-brand" href=".">tickets.io</a>

    <button
      class="navbar-toggler"
      type="button"
      data-toggle="collapse"
      data-target="#navbarSupportedContent"
      aria-controls="navbarSupportedContent"
      aria-expanded="false"
      aria-label="Toggle navigation"
    >
      <span class="navbar-toggler-icon" />
    </button>


    <div class="collapse navbar-collapse" id="navbarSupportedContent">
      <ul class="navbar-nav">
        <li class="nav-item">
          <a class="nav-link " href=".">About</a>
        </li>
        <li class="nav-item dropdown">
          <a
            class="nav-link dropdown-toggle"
            href="."
            id="navbarDropdown"
            role="button"
            data-toggle="dropdown"
            aria-haspopup="true"
            aria-expanded="false"
          >
            Project
          </a>
          <div class="dropdown-menu" aria-labelledby="navbarDropdown">
            <a class="dropdown-item" href=".">Action</a>
            <a class="dropdown-item" href=".">Another action</a>
            <div class="dropdown-divider" />
            <a class="dropdown-item" href=".">Something else here</a>
          </div>
        </li>
          <a class="nav-link disabled" href=".">Login</a>
      </ul>
    </div>
  </nav>

  <div class="container m-0">
    <div class="row">
      <nav class="nav col-2 p-3 bg-light flex-column">
        <a class="nav-link" href=".">Backlog</a>
        <a class="nav-link" href=".">Sprint</a>
        <a class="nav-link" href=".">Archive</a>
      </nav>
      <div class="col-10 flex-column p-4 flex-wrap bd-highlight">
        <h3 class="mb-4 border-bottom">Backlog</h3>
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
      </div>
    </div>
  </div>
</main>
