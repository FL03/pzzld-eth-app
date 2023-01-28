<script>
    import { info } from '$lib/constants.js';
    import { Navbar, Sidebar, Toolbar, WalletToggle } from '$lib';
    import { connected } from 'svelte-web3';

    let open = false;
</script>

<Sidebar bind:open>
	Control Center
</Sidebar>

<Navbar name={info.name}>
    <WalletToggle/>
</Navbar>
<div class="flex flex-col grow items-center justify-center min-h-full max-h-screen min-w-full max-w-screen z-0" id="content-section">
    <main class="bg-zinc-700 flex container mx-auto prose prose-invert rounded p-3">
        {#if $connected}
            <slot/>
        {/if}
    </main>
</div>
<Toolbar 
    linktree={info.sitemap.dashboard.links} 
    bind:sidebar={open}
/>

<style>
    
    
</style>