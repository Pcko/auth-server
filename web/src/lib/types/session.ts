export interface Session {
    id: string;
    uid: string;
    created_at: Date;
    expires_at: Date;
    last_seen_at: Date;
    revoked_at: Date | null;
    user_agent: string;
    ip_address: string | null;
}