<script>
    import { onMount } from 'svelte';
    import headerImage from '$lib/assets/media/abstractShapes.svg';
    import { fetch, greet, add_one } from 'wasm';
    
    let value = 0;

    onMount(() => {
        greet("Puzzled");
    })
</script>

<svelte:head>
	<title>Dashboard</title>
	<meta name="description" content="Welcome my dApp" />
</svelte:head>



<span class="items-center justify-center text-center">
    <img alt="#" class="mx-3 rounded" src={headerImage}/>
    <h1 class="prose prose-invert prose-2xl">
        Puzzled
    </h1>
</span>