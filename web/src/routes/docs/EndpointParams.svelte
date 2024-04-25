<script>
	export let params = [];
</script>

<h3 class="text-lg mb-2 font-bold">Parámetros:</h3>
<table class="w-full">
	<tr class="bg-primary-500">
		<th class="w-36">Campo</th>
		<th class="w-36">Dónde</th>
		<th class="w-36">Tipo</th>
	</tr>
	{#each params as param}
		<tr class="border-b" height="35">
			<td class="text-left"><pre>{param.campo}</pre></td>
			<td class="text-center"><pre>{param.tipo}</pre></td>
			<td class="text-center">{param.donde}</td>
		</tr>
	{/each}
</table>
