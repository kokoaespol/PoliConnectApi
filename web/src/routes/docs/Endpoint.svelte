<script>
	import EndpointParams from './EndpointParams.svelte';

	export let title = '';
	export let method = 'GET';
	export let endpoint = '';
	export let params = [];
</script>

<h3 class="text-lg mb-2 font-bold">{title}</h3>
<div class="flex items-stretch w-full bg-gray-900 font-mono my-4 gap-2">
	<div class="w-1 bg-primary-500 inline-block"></div>
	{method}
	{endpoint}
</div>

<EndpointParams {params} />

<div class="mt-4">
	<span class="text-lg mb-2 font-bold">Ejemplo de respuesta:</span>
	<div class="border p-2 mt-4">
		<slot />
	</div>
</div>
