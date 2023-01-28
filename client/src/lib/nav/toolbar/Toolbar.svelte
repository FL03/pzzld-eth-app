<script>
    import { Searchbar, SidebarToggle } from '$lib/nav';

    
    import { page } from '$app/stores';
    export let linktree = [];
    export let sidebar = false;

    let search = "";
</script>

<div class="bg-transparent prose prose-invert flex flex-nowrap items-center justify-between max-h-fit min-w-full max-w-screen m-0 px-3 py-1 z-50 sticky bottom-0">
    <div class="divide-x flex">
        <div class="px-3 items-center hidden lg:flex">
            <SidebarToggle bind:open={sidebar}/>
        </div>
        <div class="flex grow items-center justify-start px-3">
            <ul class="flex grow items-center justify-start list-none hidden lg:flex">
                {#each linktree as view}
                    <li class:active={$page.url.pathname === view.href}>
                        <a class="px-3 hover:opacity-75 hover:italic hover:underline" sveltekit:prefetch href="{view.href}">
                            {view.label}
                        </a>
                    </li>
                {/each}
            </ul>
        </div>
        
    </div>
    <Searchbar bind:query={search}/>
</div>

<style>
    
</style>