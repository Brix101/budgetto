import Link from "next/link";

export function DashboardHeader() {
  return (
    <header>
      Dashboard
      <Link
        href={"/api/auth/signout"}
        className="transition-all text-stone-400 hover:text-stone-200"
      >
        Goddammit, sign me out!
      </Link>
    </header>
  );
}
