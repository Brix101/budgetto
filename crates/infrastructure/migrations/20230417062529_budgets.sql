-- Add migration script here
CREATE TABLE IF NOT EXISTS public.budgets (
    id uuid DEFAULT uuid_generate_v4 (),
    amount DOUBLE PRECISION NOT NULL,
    category_id UUID NOT NULL UNIQUE REFERENCES public.categories (id),
    user_id uuid NOT NULL REFERENCES public.users (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    deleted_at TIMESTAMPTZ,
    PRIMARY KEY (id),
    UNIQUE (user_id, category_id)
);
