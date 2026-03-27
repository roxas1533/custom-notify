import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { Match, Switch } from "solid-js";
import { render } from "solid-js/web";
import NotificationView from "./notification/NotificationView";
import SettingsView from "./settings/SettingsView";

const log = (msg: string) =>
	invoke("frontend_log", { message: msg }).catch(() => {});

function App() {
	let label = "";
	try {
		label = getCurrentWebviewWindow().label;
		log(`App render, label=${label}`);
	} catch (e) {
		log(`getCurrentWebviewWindow error: ${String(e)}`);
	}

	return (
		<Switch fallback={<div />}>
			<Match when={label.startsWith("notification_")}>
				<NotificationView />
			</Match>
			<Match when={label === "settings"}>
				<SettingsView />
			</Match>
		</Switch>
	);
}

try {
	const root = document.getElementById("root");
	if (root) render(() => <App />, root);
	log("render() completed");
} catch (e) {
	log(`render() error: ${String(e)}`);
}
