<script>
    export let query = "";
    
    function handle_click() {

    }
</script>

<div class="flex prose">
    <input 
    class="flex items-center rounded-full mx-3 py-1 px-3 text-black hover:opacity-95 shadow drop-shadow"
    placeholder="Search"
    value={query}
    >
    <button 
        class="bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900 prose prose-invert rounded py-1 px-2.5 hover:opacity-95 shadow drop-shadow"
        on:click={handle_click}
        >
        Search
    </button>
</div>