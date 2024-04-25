<script>
	import '../app.postcss';
	import { AppBar } from '@skeletonlabs/skeleton';
	import logoWhite from '$lib/assets/logo-white.png';
</script>

<header>
	<AppBar>
		<svelte:fragment slot="lead">
			<img src={logoWhite} alt="logo" width="50" />
		</svelte:fragment>
		<a href="/">PoliConnect</a>
		<svelte:fragment slot="trail">
			<a href="/docs">Docs</a>
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

<slot />
