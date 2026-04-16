CREATE TABLE users (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL,
    name TEXT NOT NULL,
    spotify_id TEXT NOT NULL,
    spotify_access_token TEXT,
    spotify_refresh_token TEXT,
    spotify_token_expires_at TIMESTAMP(3) WITHOUT TIME ZONE,
    avatar_url TEXT,
    daily_song_playlist_id TEXT,
    created_at TIMESTAMP(3) WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX users_email_key ON users (email);
CREATE UNIQUE INDEX users_spotify_id_key ON users (spotify_id);

CREATE TABLE submissions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON UPDATE CASCADE ON DELETE RESTRICT,
    track_id TEXT NOT NULL,
    date TEXT NOT NULL,
    mood TEXT,
    notes TEXT,
    notified_at TIMESTAMP(3) WITHOUT TIME ZONE
);

CREATE INDEX submissions_notified_at_idx ON submissions (notified_at);

CREATE TABLE comments (
    id TEXT PRIMARY KEY,
    submission_id TEXT NOT NULL REFERENCES submissions(id) ON UPDATE CASCADE ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES users(id) ON UPDATE CASCADE ON DELETE RESTRICT,
    content TEXT NOT NULL,
    created_at TIMESTAMP(3) WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE push_subscriptions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON UPDATE CASCADE ON DELETE CASCADE,
    subscription JSONB NOT NULL,
    created_at TIMESTAMP(3) WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
