import { EntityRepository } from "@mikro-orm/core";

import type { Category } from "./entities/category.entity";

export class CategoryRepository extends EntityRepository<Category> {}
