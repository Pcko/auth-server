import {
	PUBLIC_APP_URL,
	PUBLIC_SERVER_URL,
} from '$env/static/public';

export const authConfig = {
	app_url: PUBLIC_APP_URL,
	idp_url: PUBLIC_SERVER_URL,
	redirect_uri: `${PUBLIC_APP_URL}/auth/callback`,
	session_duration_minutes: 60 * 24 * 7
};