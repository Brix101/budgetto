// import { NextApiRequest, NextApiResponse } from "next";
// import NextAuth, { NextAuthOptions } from "next-auth";
// import CredentialsProvider from "next-auth/providers/credentials";
// import { RestUserModel } from "../../../types/models";
// import createAxiosRequest from "../../../utils/axios";
// import { getData, postData } from "../../../utils/request";
//
// type NextAuthOptionsCallback = (
//   req: NextApiRequest,
//   res: NextApiResponse
// ) => NextAuthOptions;
//
// const nextAuthOptions: NextAuthOptionsCallback = (
//   req: NextApiRequest,
//   res: NextApiResponse
// ): NextAuthOptions => {
//   return {
//     providers: [
//       CredentialsProvider({
//         name: "Credentials",
//         credentials: {
//           email: { label: "Email", type: "email", placeholder: "jhon@doe.com" },
//           password: { label: "Password", type: "password" },
//         },
//         async authorize(credentials) {
//           try {
//             const response = await postData<RestUserModel>("/auth/signin", {
//               body: {
//                 email: credentials?.email,
//                 password: credentials?.password,
//               },
//             });
//
//             console.info("Authorization was successful!");
//
//             const cookies = response.headers["set-cookie"] || [];
//             res.setHeader("Set-Cookie", cookies);
//
//             return response.data;
//           } catch (error: any) {
//             console.error(`NextAuth authorize error: ${error.message}`);
//             throw error;
//           }
//         },
//       }),
//     ],
//     callbacks: {
//       // async signIn({ user }) {
//       //   if (!(user.role === "admin" || user.role === "manager")) return false;
//       //   return true;
//       // },
//       async jwt({ token, user }) {
//         if (user) {
//           return { user };
//         }
//
//         // I suppose that if the cookie expires it's just a simple check of whether it still exists or not
//         if (req.cookies["Authentication"]) return token;
//
//         try {
//           // This gives a Set-Cookie with a new Access token
//           const response = await getData<RestUserModel>(
//             "http://localhost:32000/api/auth/refresh"
//           );
//
//           console.debug("JWT Callback RETURN", token);
//
//           return {
//             ...token,
//             user: response.data,
//           };
//         } catch (error) {
//           return {
//             ...token,
//             error: "RefreshAccessTokenError",
//           };
//         }
//       },
//     },
//     session: {
//       strategy: "jwt",
//     },
//     events: {
//       async signOut() {
//         res.setHeader("Set-Cookie", [
//           "Authentication=deleted;Max-Age=0;path=/;",
//           "Refresh=deleted;Max-Age=0;path=/;",
//         ]);
//       },
//     },
//   };
// };
//
// const Auth = (req: NextApiRequest, res: NextApiResponse) => {
//   return NextAuth(req, res, nextAuthOptions(req, res));
// };
//
// export default Auth;
