import { SetMetadata } from "@nestjs/common";

import { IS_PUBLIC_KEY } from "./auth.constants";

export const Auth = (...args: string[]) => SetMetadata("auth", args);

export const Public = () => SetMetadata(IS_PUBLIC_KEY, true);
