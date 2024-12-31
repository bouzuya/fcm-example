import { useCallback, useState, useTransition } from "react";
import { initializeApp } from "firebase/app";
import { getMessaging, getToken } from "firebase/messaging";
import "./App.css";

const firebaseConfigJson = import.meta.env.VITE_FIREBASE_CONFIG_JSON;
if (firebaseConfigJson === undefined)
	throw new Error("env VITE_FIREBASE_CONFIG_JSON not found");
const firebaseConfig = JSON.parse(firebaseConfigJson);
initializeApp(firebaseConfig);

const vapidKey = import.meta.env.VITE_VAPID_KEY;
if (vapidKey === undefined) throw new Error("env VITE_VAPID_KEY not found");

const backendBaseUrl = import.meta.env.VITE_BACKEND_BASE_URL;
if (backendBaseUrl === undefined) throw new Error("env VITE_BACKEND_BASE_URL not found");

const baseUrl = import.meta.env.BASE_URL;
if (baseUrl === undefined) throw new Error("env BASE_URL not found");
if (baseUrl === '/') throw new Error("env BASE_URL is invalid");

function getNotificationPermission(): NotificationPermission | null {
	if (!("Notification" in window)) return null;
	return Notification.permission;
}

async function registerServiceWorkerAndGetToken(): Promise<string> {
	const messaging = getMessaging();

	// register service worker
	if (!("serviceWorker" in navigator)) {
		throw new Error("serviceWorker not supported");
	}
	const serviceWorkerRegistration = await navigator.serviceWorker.register(
		`${baseUrl}/firebase-messaging-sw.js`,
	);
	await serviceWorkerRegistration.update().catch(() => {
		// ignore error
	});

	const token = await getToken(messaging, {
		serviceWorkerRegistration,
		vapidKey,
	});

	// register token
	const response = await fetch(`${backendBaseUrl}/tokens`, {
		body: JSON.stringify({ token }),
		headers: {
			'Content-Type': 'application/json'
		},
		method: "POST",
	});
	if (((response.status / 100) | 0) !== 2) {
		throw new Error("token registration failed");
	}

	return token;
}

async function writeClipboardText(s: string): Promise<void> {
	return navigator.clipboard.writeText(s);
}

function App() {
	const [token, setToken] = useState<string | null>(null);
	const [permission, setPermission] = useState<NotificationPermission | null>(
		getNotificationPermission,
	);
	const onClickCopyButton = useCallback((): void => {
		startTransition(async () => {
			try {
				if (token === null) return;
				await writeClipboardText(token);
			} catch (e) {
				console.error(e);
			}
		});
	}, [token]);
	const [isPending, startTransition] = useTransition();
	const onClickGetTokenButton = useCallback((): void => {
		startTransition(async () => {
			try {
				const token = await registerServiceWorkerAndGetToken();
				setToken(token);
				setPermission(getNotificationPermission());
			} catch (e) {
				console.error(e);
			}
		});
	}, []);
	const onClickRequestPermissionButton = useCallback((): void => {
		startTransition(async () => {
			try {
				const permission = await Notification.requestPermission();
				setPermission(permission);
			} catch (e) {
				console.error(e);
			}
		});
	}, []);
	// TODO: fetch token from server

	return (
		<div className="card">
			<div>Notification permission: {permission}</div>
			{permission === null ? (
				<div>通知はサポートされていません。</div>
			) : permission === "denied" ? (
				<div>
					通知を許可してください。
					<a href="https://support.google.com/chrome/answer/3220216">
						https://support.google.com/chrome/answer/3220216
					</a>
				</div>
			) : permission === "default" ? (
				<div>
					〜のお知らせを通知するには通知の許可が必要です。
					<button onClick={onClickRequestPermissionButton} type="button">
						通知を許可する
					</button>
				</div>
			) : (
				<div>
					<button
						disabled={isPending}
						onClick={onClickGetTokenButton}
						type="button"
					>
						{isPending ? "(処理中)" : "トークンを取得する"}
					</button>
					<div>
						{token !== null ? (
							<div>
								<input type="text" value={token} />
								<button onClick={onClickCopyButton} type="button">
									コピー
								</button>
							</div>
						) : (
							<div>(none)</div>
						)}
					</div>
				</div>
			)}
		</div>
	);
}

export default App;
