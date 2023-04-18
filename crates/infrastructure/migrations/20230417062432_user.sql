-- Add migration script here
CREATE TABLE IF NOT EXISTS public.user (
    id UUID DEFAULT uuid_generate_v4 (),
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    bio VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    PRIMARY KEY (id)
);
CREATE INDEX IF NOT EXISTS user_email_idx ON public.user (email);