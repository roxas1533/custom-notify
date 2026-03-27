export interface NotificationData {
	id: string;
	title: string;
	body: string;
	icon_url?: string;
	duration_ms: number;
	animation_duration_ms: number;
	style: string;
}

export interface Settings {
	port: number;
	notification_position:
		| "top_right"
		| "top_left"
		| "bottom_right"
		| "bottom_left";
	notification_duration_ms: number;
	max_visible_notifications: number;
	notification_width: number;
	notification_height: number;
	notification_gap: number;
	animation_duration_ms: number;
}
