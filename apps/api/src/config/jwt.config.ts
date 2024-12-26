import { registerAs } from "@nestjs/config";

export interface JwtConfig {
  accessPublicKey: string;
  accessPrivateKey: string;
  accessExpiresIn: string;
  refreshPublicKey: string;
  refreshPrivateKey: string;
  refreshExpiresIn: string;
}

export default registerAs(
  "jwt",
  (): JwtConfig => ({
    accessPublicKey: process.env.ACCESS_PUBLIC_KEY,
    accessPrivateKey: process.env.ACCESS_PRIVATE_KEY,
    accessExpiresIn: process.env.ACCESS_EXPIRES_IN,
    refreshPublicKey: process.env.REFRESH_PUBLIC_KEY,
    refreshPrivateKey: process.env.REFRESH_PRIVATE_KEY,
    refreshExpiresIn: process.env.REFRESH_EXPIRES_IN,
  }),
);
