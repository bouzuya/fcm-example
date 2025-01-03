/// <reference types="vite/client" />

interface ImportMetaEnv {
	readonly VITE_VAPID_KEY: string;
	// more env variables...
}

interface ImportMeta {
	readonly env: ImportMetaEnv;
}
