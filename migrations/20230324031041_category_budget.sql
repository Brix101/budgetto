-- Add migration script here

create table category_budget
      (category_id     integer not null
      ,budget_id      integer not null
      ,constraint to2category_fk  
                  foreign key (category_id)
                  references categories (id) 
                  on delete cascade
      ,constraint to2budget_fk  
                  foreign key (budget_id)
                  references budgets (id)
                  on delete cascade
      );
