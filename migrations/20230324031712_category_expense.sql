-- Add migration script here

create table category_expense
      (category_id     integer not null
      ,expense_id      integer not null
      ,constraint to2category_fk  
                  foreign key (category_id)
                  references categories (id) 
                  on delete cascade
      ,constraint to2expense_fk  
                  foreign key (expense_id)
                  references expenses (id)
                  on delete cascade
      );
