import { invoke } from "@tauri-apps/api/core";
import { createSignal, onMount, Show } from "solid-js";
import type { NotificationData } from "../types";
import "./notification.css";

const STYLE_ICONS: Record<string, string> = {
	info: "\u2139",
	success: "\u2713",
	warning: "\u26A0",
	error: "\u2716",
};

export default function NotificationView() {
	const [data, setData] = createSignal<NotificationData | null>(null);
	const [phase, setPhase] = createSignal<
		"entering" | "visible" | "exiting" | "gone"
	>("gone");

	onMount(async () => {
		const log = (msg: string) => invoke("frontend_log", { message: msg });
		await log("NotificationView onMount");

		// Backend calls this to trigger dismiss animation
		(window as unknown as Record<string, unknown>).__dismiss = () => {
			setPhase("exiting");
			const d = data();
			if (d) {
				setTimeout(() => setPhase("gone"), d.animation_duration_ms);
			}
		};

		let result: NotificationData | null;
		try {
			result = await invoke<NotificationData | null>("notification_ready");
			await log(`notification_ready: ${JSON.stringify(result)}`);
		} catch (e) {
			await log(`notification_ready error: ${String(e)}`);
			return;
		}
		if (!result) return;

		setData(result);
		setPhase("entering");
		setTimeout(() => setPhase("visible"), result.animation_duration_ms);
	});

	const handleClose = () => {
		const d = data();
		if (!d) return;
		setPhase("exiting");
		setTimeout(() => {
			invoke("close_notification", { id: d.id });
		}, d.animation_duration_ms);
	};

	return (
		<Show when={data()}>
			{(notif) => {
				const styleClass = () => `style-${notif().style}`;
				const phaseClass = () => phase();
				const icon = () => STYLE_ICONS[notif().style] ?? STYLE_ICONS.info;

				return (
					<button
						type="button"
						class={`notification-card ${phaseClass()} ${styleClass()}`}
						style={{
							"transition-duration": `${notif().animation_duration_ms}ms`,
							cursor: "pointer",
						}}
						onClick={handleClose}
					>
						<div class="notification-header">
							<Show
								when={notif().icon_url}
								fallback={<span class="style-icon">{icon()}</span>}
							>
								{(url) => <img class="notification-icon" src={url()} alt="" />}
							</Show>
							<span class="notification-title">{notif().title}</span>
							<button
								type="button"
								class="notification-close"
								onClick={handleClose}
							>
								&times;
							</button>
						</div>
						<div class="notification-body">{notif().body}</div>
					</button>
				);
			}}
		</Show>
	);
}
