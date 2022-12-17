<script>
    import { fetch, greet, add_one } from 'wasm';
    
    let value = 0;
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
        <button on:click={() => {greet("Joe")}}>Alert</button>
    </div>
</div>