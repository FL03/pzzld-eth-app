<script>
    import { selectedAccount } from 'svelte-web3';
    
    export let picture = "https://cdn.pixabay.com/photo/2015/10/05/22/37/blank-profile-picture-973460_960_720.png";
    export let bg = "bg-zinc-800";
    export let title = "Puzzled";
</script>

<div class="antialiased {bg} flex flex-wrap p-3 rounded-lg shadow-lg w-full">
    <div class="md:w-1/3 w-full">
        <img 
            alt="#"
            class="rounded-lg shadow-lg antialiased"
            src={picture}
        />  
    </div>
    <div class="md:w-2/3 w-full px-3 flex flex-row flex-wrap overflow-x-hidden">
        <div class="w-full text-right text-gray-700 font-semibold relative pt-3 md:pt-0">
            <div class="text-2xl text-white leading-tight">
                {title}
            </div>
            <div class="text-normal text-gray-300 hover:text-gray-400 cursor-pointer">
                <span class="border-b border-dashed border-gray-500 pb-1">
                    {$selectedAccount}
                </span>
            </div>
            <div class="text-sm text-gray-300 hover:text-gray-400 cursor-pointer md:absolute pt-3 md:pt-0 bottom-0 right-0">
                Last Seen: <b>2 days ago</b>
            </div>
        </div>
    </div>
</div>

