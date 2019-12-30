export type ResolveType<T> = T extends Promise<infer U> ? U : never;
