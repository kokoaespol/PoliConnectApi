<script lang="ts">
	import EndpointParam from './EndpointParam.svelte';
	import type { EndpointParam as Param } from '$lib/types/EndpointParam';

	enum Where {
		Body,
		Path,
		Query
	}

	export let where: Where = Where.Body;
	export let params: Param[];
</script>

{#if where === Where.Body}
	<h3 class="text-lg mb-2 font-bold">Cuerpo</h3>
{:else if where === Where.Path}
	<h3 class="text-lg mb-2 font-bold">Ruta</h3>
{:else}
	<h3 class="text-lg mb-2 font-bold">Parámetros</h3>
{/if}

<div class="border-b w-full"></div>
{#each params as param}
	<EndpointParam {param} />
{/each}
