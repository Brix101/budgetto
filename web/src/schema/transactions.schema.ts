import z from "zod";

export const TransactionType = z.enum([
  "Expense",
  "Income",
  "Transfer",
  "Refund",
]);

export type TransactionType = z.infer<typeof TransactionType>;

export const transactionSchema = z.object({
  id: z.string(),
  amount: z.number(),
  transactionType: TransactionType,
  note: z.string().nullish(),
  createdAt: z.string(),
  updatedAt: z.string(),
});

export const transactionsSchema = z.object({
  transactions: transactionSchema.array(),
});

export type Transaction = z.TypeOf<typeof transactionSchema>;
export type Transactions = z.TypeOf<typeof transactionsSchema>;
