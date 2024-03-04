import { json } from "@remix-run/node";

export const loader = async () => {
	return json(
		{
			theme_color: "#f69435",
			background_color: "#f69435",
			display: "standalone",
			scope: "/",
			start_url: "/",
			name: "Roll Your Own Tracker",
			short_name: "Ryot",
			description: "Track all facets of your life!",
			icons: [
				{
					src: "/icon-192x192.png",
					sizes: "192x192",
					type: "image/png",
				},
				{
					src: "/icon-256x256.png",
					sizes: "256x256",
					type: "image/png",
				},
				{
					src: "/icon-384x384.png",
					sizes: "384x384",
					type: "image/png",
				},
				{
					src: "/icon-512x512.png",
					sizes: "512x512",
					type: "image/png",
				},
			],
		},
		{
			headers: {
				"Cache-Control": "public, max-age=600",
				"Content-Type": "application/manifest+json",
			},
		},
	);
};
