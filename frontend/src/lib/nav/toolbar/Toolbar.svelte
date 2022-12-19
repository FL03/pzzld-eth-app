<script>
    import { Searchbar, SidebarToggle } from '$lib/nav';

    
    import { page } from '$app/stores';
    export let linktree = [];
    export let sidebar = false;
    let search = "";

    export let align = {
        items: "items-center",
        justify: "justify-between"
    };
    export let bg = "bg-transparent";
    export let color = "text-white";
    export let m = "m-0";
    export let pd = "p-3";
    export let position = "sticky bottom-0";
    export let dim = {
        h: "",
        w: "min-w-full max-w-screen",
        z: "z-50"
    }
</script>

<div class="flex flex-nowrap {align.items} {align.justify} {bg} {color} {dim.h} {dim.w} {dim.z} {m} {pd} {position}">
    <div class="divide-x flex grow">
        <div class="px-3 items-center hidden lg:flex">
            <SidebarToggle bind:open={sidebar}/>
        </div>
        <div class="flex grow items-center justify-start p-3">
            <ul class="flex grow items-center justify-start list-none hidden lg:flex">
                {#each linktree as view}
                    <li class:active={$page.url.pathname === view.href}>
                        <a class="flex px-3 py-2 hover:opacity-75 hover:italic hover:underline" sveltekit:prefetch href="{view.href}">
                            {view.label}
                        </a>
                    </li>
                {/each}
            </ul>
        </div>
        
    </div>
    <Searchbar bg="bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900" color={color} bind:query={search}/>
</div>

<style>
    
</style>