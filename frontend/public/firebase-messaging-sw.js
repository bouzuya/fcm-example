const version = "0.19.0";

// <https://developer.mozilla.org/ja/docs/Web/API/ServiceWorkerGlobalScope/push_event>
self.addEventListener("push", (event) => {
	// event: PushEvent
	// <https://developer.mozilla.org/en-US/docs/Web/API/PushEvent>
	// PushEvent.data: PushMessageData | null;
	const pushMessageData = event.data;
	if (pushMessageData === null) return;
	// data: unknown
	const data = pushMessageData.json() ?? {};

	// <https://firebase.google.com/docs/reference/admin/node/firebase-admin.messaging.webpushnotification.md#webpushnotification_interface>
	if (!(typeof data === "object" && data !== null && "notification" in data))
		return;
	const { title, ...options } = data.notification;
	if (title !== null && title !== undefined) {
		// <https://developer.mozilla.org/ja/docs/Web/API/ServiceWorkerRegistration/showNotification>
		self.registration.showNotification(`${title} (v${version})`, options);
	}
});

// <https://developer.mozilla.org/ja/docs/Web/API/ServiceWorkerGlobalScope/notificationclick_event>
self.addEventListener("notificationclick", (event) => {
	// event: NotificationEvent
	// <https://developer.mozilla.org/ja/docs/Web/API/NotificationEvent>

	// <https://developer.mozilla.org/ja/docs/Web/API/Notification/close>
	event.notification.close();

	const url = event.notification.data?.url ?? null;
	if (!(typeof url === "string" || url === null)) return;
	if (url !== null) {
		// <https://developer.mozilla.org/ja/docs/Web/API/ServiceWorkerGlobalScope/clients>
		// <https://developer.mozilla.org/ja/docs/Web/API/Clients/openWindow>
		event.waitUntil(clients.openWindow(url));
	}
});
