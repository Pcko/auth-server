import { redirect } from '@sveltejs/kit';
import { randomBytes } from 'node:crypto';
import {PUBLIC_APP_URL,PUBLIC_IS_DEV} from '$env/static/public';
import type { LayoutServerLoad } from './$types';
import { clearImmediate } from 'node:timers';

const APP_URL = 'http://localhost:5173';

function randomState() {
	return randomBytes(32).toString('base64url');
}

export const load: LayoutServerLoad = async ({ cookies, url }) => {
	const appSession = cookies.get('app_session');

	if (appSession) {
		// validate appSession
		return {};
	}

	// state for login-flow protection.
	const state = randomState();
	// the url the user originally wanted
	const returnTo = url.pathname + url.search;
	// where the IdP should send the user after login
	const redirectUri = `${APP_URL}/auth/callback`;
	const clientId = url.searchParams.get('client_id');

	if(!clientId) {
		throw new Error('Used Client is invalid!');
	}

	cookies.set('oauth_state', state, {
		path: '/',
		httpOnly: true,
		secure: !!PUBLIC_IS_DEV,
		sameSite: 'lax',
		maxAge: 60 * 10
	});

	cookies.set('return_to', returnTo, {
		path: '/',
		httpOnly: true,
		secure: !!PUBLIC_IS_DEV,
		sameSite: 'lax',
		maxAge: 60 * 10
	});

	const authUrl = new URL('/login', PUBLIC_APP_URL);
	authUrl.searchParams.set('client_id', clientId);
	authUrl.searchParams.set('redirect_uri', redirectUri);
	authUrl.searchParams.set('state', state);

	throw redirect(307, authUrl.toString());
};