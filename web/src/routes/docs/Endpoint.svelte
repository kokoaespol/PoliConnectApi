<script>
	import EndpointParams from './EndpointParams.svelte';
	import EndpointResponse from './EndpointResponse.svelte';

	export let title = '';
	export let method = 'GET';
	export let endpoint = '';
	export let params = [];
	export let responses = [];

	let showPopup = false;

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
</script>

{#if showPopup}
    <div class="fixed bottom-5 right-5 bg-green-500 text-white px-4 py-2 rounded shadow">
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
		class="cursor-pointer"
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
