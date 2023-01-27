<script>
    import { page } from '$app/stores';

    export let links;
</script>

<ul class="flex grow items-center justify-start list-none hidden lg:flex prose prose-invert">
    {#each links as view}
        <li class:active={$page.url.pathname === view.href}>
            <a class="block px-3 py-2 hover:opacity-75 hover:italic" sveltekit:prefetch href="{view.href}">
                {view.label}
            </a>
        </li>
    {/each}
</ul>