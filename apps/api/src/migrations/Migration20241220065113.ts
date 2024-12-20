import { Migration } from '@mikro-orm/migrations';

export class Migration20241220065113 extends Migration {

  override async up(): Promise<void> {
    this.addSql(`create table "user" ("id" serial primary key, "created_at" timestamptz not null, "updated_at" timestamptz not null, "name" varchar(255) not null, "email" varchar(255) not null, "password" varchar(255) not null, "is_confirmed" boolean not null default false);`);
    this.addSql(`alter table "user" add constraint "user_email_unique" unique ("email");`);

    this.addSql(`create table "category" ("id" serial primary key, "created_at" timestamptz not null, "updated_at" timestamptz not null, "name" varchar(255) not null, "description" varchar(255) null, "type" text check ("type" in ('income', 'expense')) not null, "user_id" int null);`);
    this.addSql(`alter table "category" add constraint "category_name_user_id_unique" unique ("name", "user_id");`);

    this.addSql(`create table "transaction" ("id" serial primary key, "created_at" timestamptz not null, "updated_at" timestamptz not null, "amount" int not null, "description" varchar(255) not null, "date" timestamptz not null, "category_id" int null, "user_id" int null);`);

    this.addSql(`create table "budget" ("id" serial primary key, "created_at" timestamptz not null, "updated_at" timestamptz not null, "amount" int not null, "description" varchar(255) not null, "start_date" timestamptz not null, "end_date" timestamptz not null, "category_id" int null, "user_id" int null);`);

    this.addSql(`alter table "category" add constraint "category_user_id_foreign" foreign key ("user_id") references "user" ("id") on update cascade on delete cascade;`);

    this.addSql(`alter table "transaction" add constraint "transaction_category_id_foreign" foreign key ("category_id") references "category" ("id") on update cascade on delete cascade;`);
    this.addSql(`alter table "transaction" add constraint "transaction_user_id_foreign" foreign key ("user_id") references "user" ("id") on update cascade on delete cascade;`);

    this.addSql(`alter table "budget" add constraint "budget_category_id_foreign" foreign key ("category_id") references "category" ("id") on update cascade on delete cascade;`);
    this.addSql(`alter table "budget" add constraint "budget_user_id_foreign" foreign key ("user_id") references "user" ("id") on update cascade on delete cascade;`);
  }

}
