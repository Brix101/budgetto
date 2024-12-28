import { cookies, headers } from "next/headers";
import NextAuth from "next-auth";
import CredentialsProvider from "next-auth/providers/credentials";

import { SignInDto, SignInResponseDto } from "@budgetto/schema";

// import { privateRoutes } from "@/contains/contants"; // an array like ["/", "/account"]

const privateRoutes = ["/account"];

// @ts-ignore
async function refreshAccessToken(token) {
  // this is our refresh token method
  console.log("Now refreshing the expired token...");
  try {
    const res = await fetch(
      `${process.env.NEXT_PUBLIC_BASE_URL}/api/auth/refresh`,
      {
        method: "POST",
        headers: await headers(),
        body: JSON.stringify({ userId: token.userId }),
      },
    );

    const { success, data } = await res.json();

    if (!success) {
      console.log("The token could not be refreshed!");
      throw data;
    }

    console.log("The token has been refreshed successfully.");

    // get some data from the new access token such as exp (expiration time)
    const decodedAccessToken = JSON.parse(
      Buffer.from(data.accessToken.split(".")[1], "base64").toString(),
    );

    return {
      ...token,
      accessToken: data.accessToken,
      refreshToken: data.refreshToken ?? token.refreshToken,
      accessTokenExpires: decodedAccessToken["exp"] * 1000,
      error: "",
    };
  } catch (error) {
    console.log(error);

    // return an error if somethings goes wrong
    return {
      ...token,
      error: "RefreshAccessTokenError", // attention!
    };
  }
}

export const { signIn, auth, handlers } = NextAuth({
  trustHost: true,
  theme: {
    logo: "https://next-auth.js.org/img/logo/logo-sm.png",
  },
  providers: [
    CredentialsProvider({
      credentials: {
        email: {
          label: "email",
          type: "email",
          placeholder: "jsmith@example.com",
        },
        password: {
          label: "password",
          type: "password",
        },
      },
      async authorize(credentials, request) {
        const cookieStore = await cookies();

        const payload = {
          email: credentials.email,
          password: credentials.password,
        } as SignInDto;

        // external api for users to log in, change it with your own endpoint
        const res = await fetch(
          `${process.env.API_BASE_URL}/api/auth/sign-in`,
          {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
              "Access-Control-Allow-Origin": "*",
            },
            body: JSON.stringify(payload),
          },
        );

        const data = await res.json();

        if (!res.ok) {
          throw new Error(data.message, { cause: data.errors });
        }

        if (res.ok && data) {
          const prefix = process.env.NODE_ENV === "development" ? "__Dev-" : "";

          // we set http only cookie here to store refresh token information as we will not append it to our session to avoid maximum size warning for the session cookie (4096 bytes)
          cookieStore.set({
            name: `${prefix}budgetto.refresh-token`,
            value: data.refreshToken,
            httpOnly: true,
            sameSite: "strict",
            secure: true,
          } as any);

          return {
            ...data,
            id: data.user.id,
          };
        }

        return null;
      },
    }),
  ],
  callbacks: {
    async jwt({ token, user, account }) {
      const _user = user as unknown as SignInResponseDto;

      if (account && _user) {
        token.id = _user.user.id;
        token.email = _user.user.email;
        token.name = _user.user.name;
        token.accessToken = _user.accessToken;
        token.refreshToken = _user.refreshToken;
        token.role = "Unknown";

        const dataToken = _user.accessToken.split(".")[1] || "";

        const decodedAccessToken = JSON.parse(
          Buffer.from(dataToken, "base64").toString(),
        );

        if (decodedAccessToken) {
          token.userId = decodedAccessToken["sub"] as string;
          token.accessTokenExpires = decodedAccessToken["exp"] * 1000;
        }
      }

      // if our access token has not expired yet, return all information except the refresh token
      if (
        (token.accessTokenExpires &&
          Date.now() < Number(token.accessTokenExpires)) ||
        token.error == "RefreshAccessTokenError"
      ) {
        const { refreshToken, ...rest } = token;
        return rest;
      }

      return await refreshAccessToken(token);
    },
    async session({ session, token, user }) {
      return {
        ...session,
        user: {
          ...session.user,
          id: token.id as string,
          name: token.name as string,
          email: token.email as string,
          accessToken: token.accessToken as string,
          accessTokenExpires: token.accessTokenExpires as number,
        },
        error: token.error,
      };
    },
  },
  // this is required
  secret: process.env.AUTH_SECRET,
  // our custom login page
  pages: {
    signIn: "/sign-in",
  },
  debug: process.env.NODE_ENV === "development",
});
