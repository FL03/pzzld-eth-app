import adapter from '@sveltejs/adapter-node';
import adapter_vercel from '@sveltejs/adapter-vercel';
import preprocess from "svelte-preprocess";

const config = {
	kit: {
		...(process.env.MODE !== "vercel") && {
			adapter: adapter()
		},
		...(process.env.MODE === "vercel") && {
			adapter: adapter_vercel()
		},
	},
	preprocess: [
		preprocess({
			postcss: true,
		}),
	],
};

export default config;
