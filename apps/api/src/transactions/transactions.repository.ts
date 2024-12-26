import { EntityRepository } from "@mikro-orm/core";

import type { Transaction } from "./entities/transaction.entity";

export class TransactionRepository extends EntityRepository<Transaction> {}
