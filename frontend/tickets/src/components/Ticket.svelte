<script lang="ts">
  export let title: string;
  export let description: string;
  let upper_title: string = title.toUpperCase();
</script>

<div class=Ticket>
  <h3>{upper_title}</h3>
  <p>{description}</p>
</div>

<style>
  div {
    text-align: left;
    border: 2px solid #4e4e5e;
    border-radius: 5px;
    padding: 1em;
    margin: 0.5em;
    max-width: 12em;
  }
  div:hover {
    background-color: #aeaebe;
    color: white;
  }
  div > h3 {
    border-bottom: 1px solid #aeaebe;
  }
  div:hover > h3 {
    border-bottom: 1px solid white;
  }
</style>