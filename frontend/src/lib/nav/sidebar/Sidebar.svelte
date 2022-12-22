<script>
    import { page } from '$app/stores';
    import { ProfileCard } from "$lib";
    export let open = false;
    export let title = "Controls";
	export const links = [["settings", "/settings"]];
</script>


<aside class="absolute bg-zinc-700 text-white flex max-w-md w-full items-center justify-between h-full opacity-95 top-0 py-24 shadow-lg drop-shadow" class:open>
    <div class="flex flex-col justify-between h-full w-full">
        <div class="flex items-center justify-center p-3">
           <ProfileCard/>
        </div>
        <div class="divide-y h-full">
            <div class="flex p-3 mx-auto items-center justify-center">
                {title}
            </div>
            <div class="flex p-3 mx-auto items-center justify-start">
                <ul class="flex flex-col lg:flex-row list-none">
                    {#each links as view}
                        <li class:active={$page.url.pathname === view[1]}>
                            <a class="block px-3 py-2 hover:opacity-75 hover:italic" sveltekit:prefetch href="{view[1]}">
                                {view[0]}
                            </a>
                        </li>
                    {/each}
                </ul>
            </div>
        </div>
    </div>
</aside>

<style>
    aside {
        left: -100%;
        transition: left 0.3s ease-in-out
    }

    .open {
        left: 0
    }
</style>
