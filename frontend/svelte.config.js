import adapter_cloudflare from '@sveltejs/adapter-node';
import adapter from '@sveltejs/adapter-node';
import preprocess from "svelte-preprocess";

const config = {
	kit: {
		...(process.env.MODE === "cloudflare") && {
			adapter: adapter_cloudflare()
		},
		...(process.env.MODE !== "cloudflare") && {
			adapter: adapter()
		}
	},
	preprocess: [
		preprocess({
			postcss: true,
		}),
	],
};

export default config;
