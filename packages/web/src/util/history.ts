import * as qs from 'qs';
import { createSelector } from '@reduxjs/toolkit';
import { createBrowserHistory } from 'history';

const history = createBrowserHistory();

export type AppQuery = {
  q?: string;
};

export default history;

export const replace = (path = '/', query?: object): void =>
  history.replace(
    [path, query ? qs.stringify(query) : ''].filter(Boolean).join('?'),
  );

const internalSelectQuery = createSelector(
  (query: string) => query,
  query => qs.parse(query) as AppQuery,
);

export const selectQuery = (): AppQuery =>
  internalSelectQuery(history.location.search.slice(1));
