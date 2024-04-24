<script>
	import { AppBar } from '@skeletonlabs/skeleton';
	import logo from '$lib/assets/logo.png';
	import logoWhite from '$lib/assets/logo-white.png';
</script>

<header>
	<AppBar>
		<svelte:fragment slot="lead">
			<img src={logoWhite} alt="logo" width="50" />
		</svelte:fragment>
		PoliConnect
		<svelte:fragment slot="trail">
			<a href="https://instagram.com/kokoa_espol" target="_blank">
				<i class="fa-brands fa-instagram"></i>
			</a>
			<a href="https://x.com/kokoa_espol" target="_blank">
				<i class="fa-brands fa-twitter"></i>
			</a>
			<a href="https://github.com/kokoaespol" target="_blank">
				<i class="fa-brands fa-github"></i>
			</a>
		</svelte:fragment>
	</AppBar>
</header>
<main class="flex flex-col p-2 mx-auto w-4/5">
	<h1 class="text-2xl mb-6">¡Bienvenido a PoliConnect!</h1>
	<section>
		<h2 class="mb-2">¿Quiénes somos?</h2>
		<p>
			Nuestro <span class="font-bold">objetivo</span> es unificar a la comunidad tecnológica de la universidad
			bajo una única organización que trabaje en equipo de manera cohesiva para proveer a la comunidad
			politécnica de soluciones tecnológicas de la más alta calidad. Esto a través del desarrollo de
			una API por y para los estudiantes de la ESPOL que abarque diferentes aspectos de la vida académica
			estudiantil.
		</p>
	</section>
	<section class="mt-6">
		<h2 class="mb-2">¿Cómo beneficiamos a la sociedad politécnica?</h2>
		<ul>
			<li class="list-disc">
				Facilitando a la comunidad politécnica acceso a información relevante sobre los diferentes
				servicios que se ofrecen en la universidad
			</li>
			<li class="list-disc">Otorgándote plataformas para que promuevas tu emprendimiento</li>
			<li class="list-disc">Promoviendo tus ayudantías</li>
		</ul>
	</section>
	<section class="mt-6">
		<h2 class="mb-2">¿Cómo participar?</h2>
		<p>
			Si quieres contribuir a PoliConnect, ponte en contacto con nostros a través de nuestro correo
			electrónico <a href="mailto:kokoa.espol.ec@gmail.com">kokoa.espol.ec@gmail.com</a> o a través de
			nuestras redes sociales.
		</p>
	</section>
	<section class="flex justify-center mt-4">
		<button type="button" class="btn variant-filled">
			<a href="https://github.com/kokoaespol/PoliConnectApi/" target="_blank">¡Participa ya!</a>
		</button>
	</section>
</main>

<footer class="p-2 flex flex-col items-center justify-center">
	<img src={logo} alt="logo" width="100" />
</footer>
