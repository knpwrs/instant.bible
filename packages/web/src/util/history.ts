import * as React from 'react';
import * as qs from 'qs';
import { createSelector } from '@reduxjs/toolkit';
import { createBrowserHistory } from 'history';

const history = createBrowserHistory();

export type AppQuery = {
  q?: string;
};

export default history;

export const replace = (path: string, query?: object): void =>
  history.replace(
    [path, query ? qs.stringify(query) : ''].filter(Boolean).join('?'),
  );

const internalSelectQuery = createSelector(
  (query: string) => query,
  query => qs.parse(query) as AppQuery,
);

export const selectQuery = (): AppQuery =>
  internalSelectQuery(history.location.search.slice(1));

export const useQuery = (): AppQuery => {
  const [query, setQuery] = React.useState(selectQuery());

  React.useEffect(
    () =>
      history.listen(() => {
        setQuery(selectQuery());
      }),
    [],
  );

  return query;
};
