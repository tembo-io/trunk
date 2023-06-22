export type Extension = {
  slug: string;
  name: string;
  description: string;
  categories: string[];
  tags: string[];
  owners: Owner[];
};

export type Owner = {
  useId: string;
  userName: string;
};
export type Category = {
  slug: string;
  name: string;
  extension_count: number;
};
export type CategoriesForGrid = {
  [key: string]: {
    displayName: string;
  };
};
