<script>
    import { onMount } from 'svelte';

    import EndpointParams from './EndpointParams.svelte';
    import EndpointResponse from './EndpointResponse.svelte';

    export let title = '';
    export let method = 'GET';
    export let endpoint = '';
    export let params = [];
    export let responses = [];

    let showPopup = false;
    let popupX = 0;
    let popupY = 0;

    function copyToClipboard(text) {
        navigator.clipboard.writeText(text)
            .then(() => {
                showPopup = true;
                setTimeout(() => {
                    showPopup = false;
                }, 1500); // 1.5s popup
            })
            .catch(err => {
                console.error('Error al copiar al portapapeles: ', err);
            });
    }

    onMount(() => {
        const buttonRect = document.querySelector('.copy-button').getBoundingClientRect();
        popupX = buttonRect.right + 10;
        popupY = buttonRect.top - 10;
    });
</script>

{#if showPopup}
    <div style="top: {popupY}px; left: {popupX}px;" class="popup">
        Copiado
    </div>
{/if}

<h3 class="text-lg mb-2 font-bold">{title}</h3>
<div class="flex justify-between items-center w-full bg-gray-900 font-mono my-4">
    <div class="flex items-stretch gap-2">
        <div class="w-1 bg-primary-500 inline-block"></div>
        {method}
        {endpoint}
    </div>
    <button
        on:click={() => copyToClipboard(`${method} ${endpoint}`)}
        class="copy-button cursor-pointer"
    >
        <i class="fa-solid fa-clipboard mr-2"></i>
    </button>
</div>

<EndpointParams {params} />

<div class="mt-4">
    {#each responses as response}
        <EndpointResponse
            params={response.params}
            status={response.status}
            contentType={response.contentType}
        />
    {/each}
</div>

<style>
    .popup {
        position: fixed;
        background-color: #4CAF50;
        color: white;
        padding: 10px;
        border-radius: 5px;
        box-shadow: 0 2px 5px rgba(0,0,0,0.2);
    }
</style>
