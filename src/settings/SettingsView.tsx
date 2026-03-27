import { invoke } from "@tauri-apps/api/core";
import { createSignal, onMount, Show } from "solid-js";
import type { Settings } from "../types";
import "./settings.css";

export default function SettingsView() {
	const [settings, setSettings] = createSignal<Settings | null>(null);
	const [saved, setSaved] = createSignal(false);

	onMount(async () => {
		const s = await invoke<Settings>("get_settings");
		setSettings(s);
	});

	const update = <K extends keyof Settings>(key: K, value: Settings[K]) => {
		setSettings((prev) => (prev ? { ...prev, [key]: value } : null));
	};

	const handleSave = async () => {
		const s = settings();
		if (!s) return;
		await invoke("save_settings", { settings: s });
		setSaved(true);
		setTimeout(() => setSaved(false), 2000);
	};

	return (
		<div class="settings-container">
			<h1 class="settings-heading">設定</h1>
			<Show when={settings()}>
				{(s) => (
					<form
						class="settings-form"
						onSubmit={(e) => {
							e.preventDefault();
							handleSave();
						}}
					>
						<label class="settings-label">
							HTTP ポート
							<input
								type="number"
								class="settings-input"
								value={s().port}
								onInput={(e) =>
									update("port", parseInt(e.currentTarget.value, 10))
								}
								min={1024}
								max={65535}
							/>
							<span class="settings-hint">
								反映にはアプリの再起動が必要です
							</span>
						</label>

						<label class="settings-label">
							通知の表示位置
							<select
								class="settings-select"
								value={s().notification_position}
								onChange={(e) =>
									update(
										"notification_position",
										e.currentTarget.value as Settings["notification_position"],
									)
								}
							>
								{(
									[
										["bottom_right", "右下"],
										["bottom_left", "左下"],
										["top_right", "右上"],
										["top_left", "左上"],
									] as const
								).map(([value, label]) => (
									<option value={value}>{label}</option>
								))}
							</select>
						</label>

						<label class="settings-label">
							表示時間 (ミリ秒)
							<input
								type="number"
								class="settings-input"
								value={s().notification_duration_ms}
								onInput={(e) =>
									update(
										"notification_duration_ms",
										parseInt(e.currentTarget.value, 10),
									)
								}
								min={1000}
								max={30000}
								step={500}
							/>
						</label>

						<label class="settings-label">
							最大同時表示数
							<input
								type="number"
								class="settings-input"
								value={s().max_visible_notifications}
								onInput={(e) =>
									update(
										"max_visible_notifications",
										parseInt(e.currentTarget.value, 10),
									)
								}
								min={1}
								max={10}
							/>
						</label>

						<label class="settings-label">
							通知の幅 (px)
							<input
								type="number"
								class="settings-input"
								value={s().notification_width}
								onInput={(e) =>
									update(
										"notification_width",
										parseInt(e.currentTarget.value, 10),
									)
								}
								min={200}
								max={600}
							/>
						</label>

						<label class="settings-label">
							通知の高さ (px)
							<input
								type="number"
								class="settings-input"
								value={s().notification_height}
								onInput={(e) =>
									update(
										"notification_height",
										parseInt(e.currentTarget.value, 10),
									)
								}
								min={60}
								max={300}
							/>
						</label>

						<button type="submit" class="settings-save-btn">
							{saved() ? "保存しました" : "保存"}
						</button>
					</form>
				)}
			</Show>
		</div>
	);
}
