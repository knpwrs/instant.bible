import { createSlice, PayloadAction, createSelector } from '@reduxjs/toolkit';
import { useSelector } from 'react-redux';
import { AppThunk, RootState } from './';
import * as api from '../util/api';
import { ResolveType } from '../util/ts';

export type ResType = ResolveType<ReturnType<typeof api.search>>;

export type QueryContainer = {
  key: string;
  inFlight: boolean;
  res?: ResType;
};

export type SliceState = {
  dirty: boolean;
  queries: {
    [key: string]: QueryContainer;
  };
};

const initialState: SliceState = {
  dirty: false,
  queries: {},
};

const { actions, reducer } = createSlice({
  name: 'search',
  initialState,
  reducers: {
    startQuery: (state, { payload }: PayloadAction<{ q: string }>): void => {
      state.dirty = true;

      const { q } = payload;

      if (!state.queries[q]) {
        state.queries[q] = { key: q, inFlight: true };
      }
    },
    endQuery: (
      state,
      { payload }: PayloadAction<{ q: string; res: ResType }>,
    ): void => {
      const { q, res } = payload;
      const sq = state.queries[q];
      if (sq) {
        sq.inFlight = false;
        sq.res = res;
      }
    },
  },
});

export default reducer;

export const { startQuery, endQuery } = actions;

export const doSearch = (q: string): AppThunk => async (
  dispatch,
  getState,
): Promise<void> => {
  const { search: searchState } = getState();

  if (searchState.queries[q]) {
    return;
  }

  dispatch(startQuery({ q }));

  const res = await api.search(q);

  dispatch(endQuery({ q, res }));
};

export const useDirty = (): boolean =>
  useSelector(({ search }: RootState) => search.dirty);

export const useLoading = (q = ''): boolean => {
  const query = useSelector(({ search }: RootState) => search.queries[q]);
  return query ? query.inFlight : false;
};

const selectResults = createSelector(
  (_state: RootState, q: string) => q,
  (state: RootState) => state.search.queries,
  (q, queries) => {
    for (let i = 0; i < q.length; i += 1) {
      const query = queries[q.substr(0, q.length - i)];
      if (query && !query.inFlight) {
        return query;
      }
    }

    return null;
  },
);

export const useRestedQuery = (q = ''): QueryContainer | null => {
  const query = useSelector((state: RootState) => selectResults(state, q));

  if (query) {
    return query;
  }

  return null;
};
