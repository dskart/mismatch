import { Container, getRandom } from "@cloudflare/containers";

// Number of container instances requests are spread across. The app is
// stateless (daily word and high scores live client-side), so any instance
// can serve any request.
const INSTANCE_COUNT = 2;

export class MismatchContainer extends Container<Env> {
	// Must match the port in the Dockerfile CMD (`serve --port 8080`)
	defaultPort = 8080;
	sleepAfter = "10m";
	envVars = {
		MISMATCH__APP__MODEL: "potion-base-8M",
		MISMATCH__APP__STORE__BLOB_STORAGE: "LOCAL",
	};

	override onError(error: unknown) {
		console.error("Container error:", error);
	}
}

export default {
	async fetch(request: Request, env: Env): Promise<Response> {
		const container = await getRandom(env.MISMATCH, INSTANCE_COUNT);
		return container.fetch(request);
	},
} satisfies ExportedHandler<Env>;
