import { baseApi } from "@/constant/server";
import { transactionsSchema } from "@/schema/transactions.schema";
import { getServerAuthSession } from "@/utils/auth";

async function getData() {
  const session = await getServerAuthSession();
  const res = await fetch(`${baseApi}/transactions`, {
    credentials: "include",
    headers: {
      Authorization: `Bearer ${session?.user.accessToken}`,
      cookie: `x-refresh=${session?.user.refreshToken}`,
    },
  });
  // The return value is *not* serialized
  // You can return Date, Map, Set,
  // Recommendation: handle errors
  if (!res.ok) {
    // This will activate the closest `error.js` Error Boundary
    throw new Error("Failed to fetch data");
  }

  return res.json().then((data) => transactionsSchema.parse(data));
}

export default async function Page() {
  const { transactions } = await getData();
  return (
    <div className="flex flex-col justify-between p-24 min-h-screen">
      {transactions?.map((transaction, index) => {
        return (
          <div key={index}>
            <h1 className="font-bold">{transaction.id}</h1>
            <span>{transaction.amount}</span>
            <p>{transaction.transactionType}</p>
            <p>{transaction.createdAt}</p>
            <p>{transaction.updatedAt}</p>
          </div>
        );
      })}
    </div>
  );
}
