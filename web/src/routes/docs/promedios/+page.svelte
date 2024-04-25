<script>
	import Endpoint from '../Endpoint.svelte';

	const calcularPromedioParams = [
		{
			campo: 'primer_parcial',
			donde: 'cuerpo',
			tipo: 'number'
		},
		{
			campo: 'segundo_parcial',
			donde: 'cuerpo',
			tipo: 'number'
		},
		{
			campo: 'mejoramiento',
			donde: 'cuerpo',
			tipo: 'number'
		},
		{
			campo: 'practico',
			donde: 'cuerpo',
			tipo: 'number'
		},
		{
			campo: 'porcentaje_practico',
			donde: 'cuerpo',
			tipo: 'number'
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
			>
				<pre>
OK 200; application/json
&lbrace;
  "missing": number | null,
  "pass": boolean,
  "grade": number
&rbrace;</pre>
			</Endpoint>
		</section>
	</section>
</main>
