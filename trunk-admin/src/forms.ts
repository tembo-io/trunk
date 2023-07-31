export interface Extension {
    categories:    string[];
    createdAt:     string;
    description:   string;
    documentation: string;
    homepage:      string;
    latestVersion: string;
    license:       string;
    name:          string;
    owners:        Owner[];
    repository:    string;
    updatedAt:     string;
}

export interface Owner {
    userId:   string;
    userName: string;
}