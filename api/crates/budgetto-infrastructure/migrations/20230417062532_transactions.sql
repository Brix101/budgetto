-- Add migration script here
DROP TYPE IF EXISTS transaction_type;
CREATE TYPE transaction_type AS ENUM ('Expense', 'Income', 'Transfer', 'Refund');
CREATE TABLE IF NOT EXISTS public.transactions (
    id uuid DEFAULT uuid_generate_v4 (),
    amount DOUBLE PRECISION NOT NULL,
    note TEXT,
    transaction_type transaction_type NOT NULL,
    account_id UUID NOT NULL,
    category_id UUID NOT NULL,
    user_id uuid NOT NULL REFERENCES public.users (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    deleted_at TIMESTAMPTZ,
    PRIMARY KEY (id)
);
