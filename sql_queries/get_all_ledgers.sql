SELECT  ledgers.id, ledgers.name,ledgers.amount,ledgers.description ,ledgers.type,ledgers.category,ledgers.created_at,ledgers.updated_at
FROM users
INNER JOIN ledgers ON ledgers.user_id = users.id
WHERE users.id = 1;
