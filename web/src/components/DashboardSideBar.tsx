"use client";
import Link from "next/link";
import { useState } from "react";

export function DashboardSideBar() {
  const [isOpen, setIsOpen] = useState(true);
  return (
    <div
      className={`flex w-64 flex-shrink-0 flex-col bg-green-200 drop-shadow-2xl transition-all duration-300 ${!isOpen && "-ml-48 justify-start"
        }`}
    >
      <ul className="bg-red-50">
        <li>
          <button
            className="w-full bg-blue-400"
            onClick={() => setIsOpen((old) => !old)}
          >
            Toggle
          </button>
        </li>
        <li className="w-full h-10">
          <Link
            href="/dashboard"
            className="flex-1 bg-blue-500 hover:bg-blue-600"
          >
            Dashboard
          </Link>
        </li>
        <li>
          <Link href="/dashboard/accounts">Accounts</Link>
        </li>
        <li>
          <Link href="/dashboard/budgets">Budgets</Link>
        </li>
        <li>
          <Link href="/dashboard/categories">Categories</Link>
        </li>
        <li>
          <Link href="/dashboard/transactions">Transactions</Link>
        </li>
      </ul>
    </div>
  );
}
