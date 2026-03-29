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