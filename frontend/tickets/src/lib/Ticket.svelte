<script lang="ts">
  export let title: string;
  export let description: string;
</script>

<div class=Ticket>
  <h3>{title}</h3>
  <p>{description}</p>
</div>

<style>
  div {
    border: 2px solid #4e4e5e;
    border-radius: 5px;
    padding: 5px;
    margin: 5px;
  }
</style>