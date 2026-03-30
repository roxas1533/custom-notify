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

const ENTER_EASING = "cubic-bezier(0.22, 1, 0.36, 1)";
const EXIT_EASING = "cubic-bezier(0.55, 0, 1, 0.45)";

function animateEnter(el: HTMLElement, durationMs: number) {
	el.animate(
		[
			{ opacity: 0, transform: "translateX(100%)" },
			{ opacity: 1, transform: "translateX(0)" },
		],
		{ duration: durationMs, fill: "forwards", easing: ENTER_EASING },
	);
}

function animateExit(el: HTMLElement, durationMs: number): Promise<void> {
	return new Promise((resolve) => {
		const anim = el.animate(
			[
				{ opacity: 1, transform: "translateX(0)" },
				{ opacity: 0, transform: "translateX(100%)" },
			],
			{ duration: durationMs, fill: "forwards", easing: EXIT_EASING },
		);
		anim.onfinish = () => resolve();
	});
}

export default function NotificationView() {
	const [data, setData] = createSignal<NotificationData | null>(null);
	let cardRef: HTMLButtonElement | undefined;
	let dismissing = false;

	onMount(async () => {
		const log = (msg: string) => invoke("frontend_log", { message: msg });
		await log("NotificationView onMount");

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

		// Backend calls this to trigger dismiss animation
		(window as unknown as Record<string, unknown>).__dismiss = () => {
			if (dismissing || !cardRef) return;
			dismissing = true;
			animateExit(cardRef, result.animation_duration_ms);
		};

		// Enter animation on next frame (DOM must be ready)
		requestAnimationFrame(() => {
			if (cardRef) animateEnter(cardRef, result.animation_duration_ms);
		});
	});

	const handleClose = async () => {
		const d = data();
		if (!d || dismissing || !cardRef) return;
		dismissing = true;
		await animateExit(cardRef, d.animation_duration_ms);
		invoke("close_notification", { id: d.id });
	};

	return (
		<Show when={data()}>
			{(notif) => {
				const styleClass = () => `style-${notif().style}`;
				const icon = () => STYLE_ICONS[notif().style] ?? STYLE_ICONS.info;

				return (
					<button
						type="button"
						ref={cardRef}
						class={`notification-card ${styleClass()}`}
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
