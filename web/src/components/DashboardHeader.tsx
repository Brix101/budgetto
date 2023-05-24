"use client";

import { signOut } from "next-auth/react";

export function DashboardHeader() {
  return (
    <header>
      Dashboard
      <button
        className="transition-all text-stone-400 hover:text-stone-200"
        onClick={() => signOut()}
      >
        Goddammit, sign me out!
      </button>
    </header>
  );
}
