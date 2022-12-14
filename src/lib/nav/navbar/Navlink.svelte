<script>
    import { page } from '$app/stores';

    export const links = [];
</script>

<ul class="flex grow items-center justify-start list-none hidden lg:flex prose prose-invert">
    {#each links as view}
        <li class:active={$page.url.pathname === view.endpoint}>
            <a class="block px-3 py-2 hover:opacity-75 hover:italic" sveltekit:prefetch href="{view.endpoint}">
                {view.label}
            </a>
        </li>
    {/each}
</ul>