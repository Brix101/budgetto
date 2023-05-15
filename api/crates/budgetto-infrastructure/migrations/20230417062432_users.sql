-- Add migration script here
CREATE TABLE IF NOT EXISTS public.users (
    id UUID DEFAULT uuid_generate_v4 (),
    name VARCHAR NOT NULL DEFAULT '',
    email VARCHAR NOT NULL DEFAULT '',
    password VARCHAR NOT NULL DEFAULT '',
    bio VARCHAR NOT NULL DEFAULT '',
    image VARCHAR NOT NULL DEFAULT '',
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY (id)
);
CREATE INDEX IF NOT EXISTS user_email_idx ON public.users (email);
