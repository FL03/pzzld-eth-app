<script>
    import { enhance } from "$app/forms";
    
    let password = "";
    let username = "";
</script>


<div class="bg-transparent flex flex-auto flex-col  items-center justify-center min-h-full min-w-full">
    <span class="flex prose prose-invert prose-2xl">
        Login
    </span>
    <form 
        class="p-3 prose rounded"
        method="POST"
        use:enhance
    >
        <input
            class="flex px-3 py-2 rounded shadow-lg my-3"
            placeholder="Username"
            value={username}
        />
        <input
            class="flex px-3 py-2 rounded shadow-lg my-3"
            placeholder="Password"
            value={password}
        />
        <div class="divide-x">
            <button>
                Login
            </button>
        </div>
    </form>
</div>

<style>

</style>