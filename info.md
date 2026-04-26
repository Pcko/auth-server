# Project: Pcko IDP


TODOs:

logoff:
change flow to
- read access token if present
- verify it if still valid
- extract sid
- read refresh token, find session in DB
- check that session.id == sid
- then revoke

session: 
restrict to:
- one per page 

IDP Structure:
IdP frontend /login:
- no fixed client_id
- reads client_id from query params
- validates it with Rust backend
- submits it back during login

Actual apps:
- each app has its own PUBLIC_CLIENT_ID env var

Rust IdP:
- stores all allowed clients and redirect URIs
- rejects unknown client_id or invalid redirect_uri