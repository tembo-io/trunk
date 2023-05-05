export interface ExtensionListing {
  name: string;
  description: string;
  author: string;
  authorLink: string;
  isInstalled?: boolean;
  createdAt?: Date;
  latestVersion?: string;
  updatedAt?: Date;
  homepage?: string;
  repository?: string;
}
