<script>
    import { fetch, greet, add_one } from 'wasm/wasm';

    let value = 0;
    function handle_click() {
        value = add_one(value);
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
    <div class="flex">
        <button on:click={handle_click}>Add</button>
        <span>{value}</span>
    </div>
</div>