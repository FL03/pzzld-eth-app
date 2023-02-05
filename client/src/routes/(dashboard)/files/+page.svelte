<script>
    import { enhance } from "$app/forms";

    let file;

    function read_file(body) {
        let reader = new FileReader();
        reader.readAsDataURL(body);
        reader.onload = (f) => {
            return f.target.result
        }
    }

</script>

<form class="flex flex-col items-center" use:enhance>
    <input
        class="flex px-3 py-2 rounded shadow-lg my-3"
        placeholder="File Upload"
        type="file"
        value={file}
    />
</form>

<style>

</style>
