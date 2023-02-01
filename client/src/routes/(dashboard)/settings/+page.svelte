<script>
    import { enhance } from "$app/forms";

    let account = {
        password: "",
        username: ""
    }
</script>

<form 
    class="p-3 prose rounded"
    method="POST"
    use:enhance
>
    <input  
            class="flex px-3 py-2 rounded shadow-lg my-3"
            placeholder="Username"
            value={account.username}
        />
        <input
            class="flex px-3 py-2 rounded shadow-lg my-3"
            placeholder="Password"
            value={account.password}
        />
</form>