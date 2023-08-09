<script>
  import { invoke } from "@tauri-apps/api/tauri";

  let name = '';
  let greetMsg = '';

  async function greet(event) {
    const name = event.target.name.value;
    
    greetMsg = await invoke('greet', { name});
  }
</script>


  <form on:submit={greet}>
  <input id="greet-input" name="name" placeholder="Enter a name" bind:value={name}/>
  <button>Greet</button>
  
    {#if greetMsg}
      <p>{greetMsg}</p>
    {/if}
</form>
 

