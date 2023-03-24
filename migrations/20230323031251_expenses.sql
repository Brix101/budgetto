-- Add migration script here

create table if not exists expenses
(
    id          uuid DEFAULT uuid_generate_v4 (),
    amount      float         not null default 0.00,
    description varchar     not null default '',
    category_id     uuid      not null references users (id) on delete cascade,
    created_at  timestamptz not null default current_timestamp,
    updated_at  timestamptz not null default current_timestamp,
    deleted_at timestamptz default null
);

alter table expenses
    add constraint expenses_id_pk primary key (id);
