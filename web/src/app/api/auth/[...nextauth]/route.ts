import { baseApi } from "@/constant/server";
import NextAuth, { type NextAuthOptions } from "next-auth";
import CredentialsProvider from "next-auth/providers/credentials";
import { env } from "process";

export const authOptions: NextAuthOptions = {
  callbacks: {
    async jwt({ token, user }) {
      // if (user) {
      //   return { user };
      // }

      console.log({ token });

      const res = await fetch(`${baseApi}/auth/re-auth`, {
        headers: {
          Authorization: `Bearer ${token.accessToken}`,
          cookie: `x-refresh=${token.refreshToken}`,
        },
      });
      let accessToken = token.accessToken;
      const xAccessToken = res.headers.get("x-access-token");

      if (xAccessToken) {
        accessToken = xAccessToken;
      }

      const body = await res.json();

      console.log({ whoami: body, accessToken, xAccessToken });
      return { ...token, ...user, accessToken, ...body.user };
    },
    session: ({ session, token }) => ({
      ...session,
      user: {
        ...session.user,
        accessToken: token.accessToken,
        refreshToken: token.refreshToken,
      },
    }),
  },
  secret: env.NEXTAUTH_SECRET,
  session: {
    strategy: "jwt",
  },
  providers: [
    CredentialsProvider({
      credentials: {
        email: { label: "Email", type: "email" },
        password: { label: "Password", type: "password" },
      },
      async authorize(credentials) {
        try {
          const { email, password } = credentials ?? {};
          if (!email || !password) {
            throw new Error("Missing username or password");
          }
          const res = await fetch(`${baseApi}/auth/sign-in`, {
            headers: {
              Accept: "application/json",
              "Content-Type": "application/json",
            },
            method: "POST",
            body: JSON.stringify({
              email,
              password,
            }),
          });

          const xRefresh = res.headers.get("set-cookie");
          // Create an empty JSON object
          const jsonObject: { [key: string]: string | boolean } = {};

          if (xRefresh) {
            // Split the string using ";"
            const parts: string[] = xRefresh.split(";");

            // Iterate through the parts and add key-value pairs to the JSON object
            for (let i = 0; i < parts.length; i++) {
              const keyValue: string[] = parts[i].trim().split("=");
              const key: string = keyValue[0].trim();
              const value: string | boolean = keyValue[1]
                ? keyValue[1].trim()
                : true;

              // Convert the key to lowercase with the first letter in lowercase
              const lowerCaseKey: string =
                key.charAt(0).toLowerCase() + key.slice(1);

              if (lowerCaseKey === "x-refresh") {
                jsonObject["value"] = value;
              } else {
                jsonObject[lowerCaseKey] = value;
              }
            }
          }

          if (res.ok) {
            const body = await res.json();
            console.log(
              "++++++++++++++++++++++++++++++++++++++++++++++++++++++++"
            );
            console.log({ body });
            console.log(
              "++++++++++++++++++++++++++++++++++++++++++++++++++++++++"
            );
            return {
              ...body.user,
              accessToken: body.accessToken,
              refreshToken: jsonObject["value"] ?? "",
            };
          }
        } catch (error: any) {
          console.error(`NextAuth authorize error: ${error.message}`);
          throw error;
        }
      },
    }),
  ],
  events: {
    async signOut() {
      console.log(
        "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++"
      );
    },
  },
};

const handler = NextAuth(authOptions);

export { handler as GET, handler as POST };
