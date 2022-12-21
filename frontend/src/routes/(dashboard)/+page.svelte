<script>
    import { fetch, greet, add_one } from 'wasm';
    
    let value = 0;
    function handle_click() {
        greet("Joe")
    }
</script>

<svelte:head>
	<title>Dashboard</title>
	<meta name="description" content="Welcome my dApp" />
</svelte:head>


<div class="flex flex-auto items-center justify-center rounded">
    <span class="prose text-center text-white">
        <h1 class="text-2xl prose text-white">Puzzled</h1>
    </span>
    <div class="block ">
        <button on:click={() => {}}>Alert</button>
    </div>
</div>