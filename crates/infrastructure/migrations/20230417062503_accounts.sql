-- Add migration script here
CREATE TABLE IF NOT EXISTS public.accounts (
  id uuid DEFAULT uuid_generate_v4 (),
  name VARCHAR NOT NULL,
  balance DOUBLE PRECISION NOT NULL,
  note TEXT,
  user_id uuid NOT NULL REFERENCES public.users (id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
  deleted_at TIMESTAMPTZ,
  PRIMARY KEY (id)
);
