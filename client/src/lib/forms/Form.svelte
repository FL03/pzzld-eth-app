<script>
    import { enhance } from "$app/forms";

    export let method = "POST";
    export let props = "m-0 py-3";
</script>

<form 
    class="{props}" 
    method={method} 
    use:enhance
>
    <slot/>
</form>

<style>
    form {
        display: flex;
        flex-direction: column;
    }
</style>