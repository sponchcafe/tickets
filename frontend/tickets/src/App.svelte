<script>
  let message = "";
  let code = 0;
  let x = new XMLHttpRequest();
  x.open("GET", "http://localhost:8000");
  x.onload = function(event) {
    console.log(this.status)
    console.log(this.response)
    message = this.responseText;
    code = this.status;
  }
  x.send();
</script>

<main>
  <div>
    From backend: {message} [{code}]
  </div>
</main>

<style>
</style>
