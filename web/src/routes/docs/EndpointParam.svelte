<script lang="ts">
	import type { EndpointParam } from '$lib/types/EndpointParam';

	export let param: EndpointParam;
</script>

<div class="flex flex-col gap-2 my-4">
	<div>
		<span class="text-secondary-500 font-bold">{param.campo}</span>
		<span class="bg-surface-500 rounded-md p-1 text-xs font-mono">{param.tipo}</span>
		{#if param.required}
			<span class="bg-red-800 text-red-200 bg-opacity-60 rounded-md p-1 text-xs font-mono">
				requerido
			</span>
		{/if}
	</div>
	<div>
		<p class="text-sm text-gray-400">
			{param.descripcion}
		</p>
	</div>
</div>
