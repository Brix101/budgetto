-- Add migration script here
CREATE TABLE IF NOT EXISTS public.category (
  id uuid DEFAULT uuid_generate_v4 (),
  name VARCHAR NOT NULL,
  note TEXT,
  user_id uuid REFERENCES public.user (id) ON DELETE CASCADE DEFAULT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
  updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
  deleted_at TIMESTAMPTZ,
  PRIMARY KEY (id)
);