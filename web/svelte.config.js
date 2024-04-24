import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

const dev = process.argv.includes('dev');

/** @type {import('@sveltejs/kit').Config} */
const config = {
	extensions: ['.svelte'],
	preprocess: [vitePreprocess()],

	kit: {
		appDir: 'app',
		adapter: adapter({
			fallback: 'index.html'
		}),
		paths: {
			base: dev ? '' : process.env.BASE_PATH,
		}
	}
};
export default config;
