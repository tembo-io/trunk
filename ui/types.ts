export interface ExtensionListing {
  name: string;
  description: string;
  author: string;
  authorLink: string;
  isInstalled?: boolean;
  createdAt?: string;
  latestVersion?: string;
  updatedAt?: string;
  homepage?: string;
  repository?: string;
}
