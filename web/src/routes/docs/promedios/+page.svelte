<script lang="ts">
	import Endpoint from '../Endpoint.svelte';
	import type { EndpointParam } from '$lib/types/EndpointParam';
	import type { EndpointResponse } from '$lib/types/EndpointResponse';

	const calcularPromedioParams: EndpointParam[] = [
		{
			campo: 'primer_parcial',
			donde: 'cuerpo',
			tipo: 'number',
			required: true,
			descripcion: 'La nota del primer parcial del 0 al 100'
		},
		{
			campo: 'segundo_parcial',
			donde: 'cuerpo',
			tipo: 'number',
			required: true,
			descripcion: 'La nota del segundo parcial del 0 al 100'
		},
		{
			campo: 'mejoramiento',
			donde: 'cuerpo',
			tipo: 'number',
			required: true,
			descripcion: 'La nota de mejoramiento de 0 a 100'
		},
		{
			campo: 'practico',
			donde: 'cuerpo',
			tipo: 'number',
			required: true,
			descripcion: 'La nota del práctico del 0 al 100'
		},
		{
			campo: 'porcentaje_practico',
			donde: 'cuerpo',
			tipo: 'number',
			required: true,
			descripcion: 'El porcentaje de la nota práctica del 0 al 100'
		}
	];

	const calcularPromedioResponses: EndpointResponse[] = [
		{
			status: 200,
			contentType: 'string',
			params: [
				{
					campo: 'pass',
					tipo: 'boolean',
					descripcion: 'Verdadero si la nota calculada es mayor a 60'
				},
				{
					campo: 'missing',
					tipo: 'number',
					descripcion: 'Puntaje requerido en mejoramiento para pasar la materia'
				},
				{
					campo: 'grade',
					tipo: 'number',
					descripcion: 'Nota calculada'
				}
			]
		}
	];
</script>

<main class="flex flex-col p-2 mx-auto w-4/5">
	<h1 class="text-2xl">API de Promedios</h1>
	<p>Esta API expone diferentes operaciones pertinentes al cálculo de notas.</p>

	<section class="my-2 w-full">
		<section class="w-full">
			<Endpoint
				title="Calcular promedio"
				method="POST"
				endpoint="/promedios"
				params={calcularPromedioParams}
				responses={calcularPromedioResponses}
			/>
		</section>
	</section>
</main>
