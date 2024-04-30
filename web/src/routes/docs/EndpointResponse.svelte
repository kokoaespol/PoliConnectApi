<script lang="ts">
	import EndpointParam from './EndpointParam.svelte';
	import type { EndpointParam as Param } from '$lib/types/EndpointParam';

	export let params: Param[];
	export let status: number;
	export let contentType: string;
</script>

<div class="flex w-full justify-between">
	<h3 class="text-lg font-bold mb-2">Response</h3>
	<small>{status} - {contentType}</small>
</div>

<div class="border-b w-full"></div>
{#each params as param}
	<EndpointParam {param} />
{/each}
