import react from "@vitejs/plugin-react"
import tailwindcss from "@tailwindcss/vite"
import tsConfigPaths from "vite-tsconfig-paths"

import { defineConfig } from "vite"

export default defineConfig({
	plugins: [react(), tailwindcss(), tsConfigPaths()],
	server: {
		port: 5173,
		host: "0.0.0.0",
		watch: {
			usePolling: true,
			interval: 100,
		},
		hmr: {
			clientPort: 80,
			protocol: "ws",
		},
	},
})
