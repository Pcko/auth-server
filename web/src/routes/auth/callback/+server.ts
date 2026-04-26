import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { authConfig } from '$lib/server/auth-config';

export const GET: RequestHandler = async ({ url, cookies, fetch }) => {
	const code = url.searchParams.get('code');
	const state = url.searchParams.get('state');
	const clientId = url.searchParams.get('client_id');

	const expectedState = cookies.get('oauth_state');
	const returnTo = cookies.get('return_to') ?? '/';

	if (!code || !state || !expectedState || state !== expectedState) {
		throw redirect(303, '/login-error');
	}

	const res = await fetch(`${authConfig.idp_url}/oauth/token`, {
		method: 'POST',
		headers: {
			'content-type': 'application/json'
		},
		body: JSON.stringify({
			grant_type: 'authorization_code',
			code,
			client_id: clientId,
			redirect_uri: authConfig.redirect_uri
		})
	});

	if (!res.ok) {
		throw redirect(303, '/login-error');
	}

	const body = await res.json();

	cookies.set('app_session', body.app_session_token, {
		path: '/',
		httpOnly: true,
		secure: false,
		sameSite: 'lax',
		maxAge: 60 * authConfig.session_duration_minutes
	});

	cookies.delete('oauth_state', { path: '/' });
	cookies.delete('return_to', { path: '/' });

	throw redirect(303, returnTo);
};