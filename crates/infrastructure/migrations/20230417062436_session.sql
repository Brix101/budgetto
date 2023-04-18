-- Add migration script here
CREATE TABLE IF NOT EXISTS public.session (
    id UUID DEFAULT uuid_generate_v4 (),
    exp TIMESTAMPTZ NOT NULL,
    user_id uuid NOT NULL REFERENCES public.user (id) ON DELETE CASCADE,
    user_agent VARCHAR NOT NULL,
    PRIMARY KEY (id)
);