<script lang="ts">
  export let title: string;
  export let description: string;
  let upper_title: string = title.toUpperCase();
</script>

<tr>
  <td>{upper_title}</td>
  <td>{description}</td>
</tr>
