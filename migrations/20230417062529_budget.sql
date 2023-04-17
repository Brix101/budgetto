-- Add migration script here
CREATE TABLE IF NOT EXISTS public.budget (
    id uuid DEFAULT uuid_generate_v4 (),
    name VARCHAR NOT NULL,
    note TEXT,
    amount DOUBLE PRECISION NOT NULL,
    category_id UUID NOT NULL,
    user_id uuid NOT NULL REFERENCES public.user (id) ON DELETE CASCADE,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    deleted_at TIMESTAMPTZ,
    PRIMARY KEY (id)
);