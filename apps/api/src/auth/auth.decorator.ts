import { SetMetadata } from "@nestjs/common";
import { IS_PUBLIC_KEY } from "src/common/constants";

export const Auth = (...args: string[]) => SetMetadata("auth", args);

export const Public = () => SetMetadata(IS_PUBLIC_KEY, true);
