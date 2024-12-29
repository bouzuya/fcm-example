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

function getNotificationPermission(): NotificationPermission | null {
	if (!("Notification" in window)) return null;
	return Notification.permission;
}

type TokenWithExpiration = {
	expiresAt: Date;
	token: string;
};

async function getTokenWithExpiration(): Promise<TokenWithExpiration> {
	const messaging = getMessaging();
	const token = await getToken(messaging, { vapidKey });
	const msPerDay = 60 * 60 * 24 * 1000;
	return {
		expiresAt: new Date(new Date().getTime() + 30 * msPerDay),
		token,
	};
}

async function writeClipboardText(s: string): Promise<void> {
	return navigator.clipboard.writeText(s);
}

function App() {
	const [token, setToken] = useState<TokenWithExpiration | null>(null);
	const [permission, setPermission] = useState<NotificationPermission | null>(
		getNotificationPermission,
	);
	const onClickCopyButton = useCallback((): void => {
		startTransition(async () => {
			try {
				if (token === null) return;
				await writeClipboardText(token.token);
			} catch (e) {
				console.error(e);
			}
		});
	}, [token]);
	const [isPending, startTransition] = useTransition();
	const onClickGetTokenButton = useCallback((): void => {
		startTransition(async () => {
			try {
				const token = await getTokenWithExpiration();
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
								<input type="text" value={token.token} />
								<button onClick={onClickCopyButton} type="button">
									コピー
								</button>
								<div>expires at: {token.expiresAt.toISOString()}</div>
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
