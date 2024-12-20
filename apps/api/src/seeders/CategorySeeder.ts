import type { EntityManager } from "@mikro-orm/core";
import { Seeder } from "@mikro-orm/seeder";

import { Category, CategoryType } from "../category/entities/category.entity";

export class CategorySeeder extends Seeder {
  private categories = [
    {
      name: "Groceries",
      type: CategoryType.EXPENSE,
      description: "Daily household items and food",
    },
    {
      name: "Rent",
      type: CategoryType.EXPENSE,
      description: "Monthly house or apartment rent",
    },
    {
      name: "Utilities",
      type: CategoryType.EXPENSE,
      description: "Electricity, water, and other utilities",
    },
    {
      name: "Transportation",
      type: CategoryType.EXPENSE,
      description: "Public transport and fuel",
    },
    {
      name: "Health",
      type: CategoryType.EXPENSE,
      description: "Medical and healthcare expenses",
    },
    {
      name: "Entertainment",
      type: CategoryType.EXPENSE,
      description: "Movies, concerts, and other entertainment",
    },
    {
      name: "Education",
      type: CategoryType.EXPENSE,
      description: "School and college fees",
    },
    {
      name: "Clothing",
      type: CategoryType.EXPENSE,
      description: "Apparel and accessories",
    },
    {
      name: "Personal Care",
      type: CategoryType.EXPENSE,
      description: "Personal hygiene and grooming",
    },
    {
      name: "Miscellaneous",
      type: CategoryType.EXPENSE,
      description: "Other unclassified expenses",
    },
    {
      name: "Salary",
      type: CategoryType.INCOME,
      description: "Monthly salary or wages",
    },
    {
      name: "Freelance",
      type: CategoryType.INCOME,
      description: "Income from freelance work",
    },
    {
      name: "Investments",
      type: CategoryType.INCOME,
      description: "Returns from investments",
    },
    {
      name: "Gifts",
      type: CategoryType.INCOME,
      description: "Money received as gifts",
    },
    {
      name: "Other",
      type: CategoryType.INCOME,
      description: "Other sources of income",
    },
  ];

  async run(em: EntityManager): Promise<void> {
    for (const category of this.categories) {
      em.create(Category, category);
    }
  }
}
