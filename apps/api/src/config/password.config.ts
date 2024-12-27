import { registerAs } from "@nestjs/config";

export interface PasswordConfig {
  secret: string;
}

export default registerAs("password", () => ({
  secret: process.env.PASSWORD_SECRET,
}));
