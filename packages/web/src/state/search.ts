import { createSlice, PayloadAction, createSelector } from '@reduxjs/toolkit';
import { useSelector } from 'react-redux';
import { AppThunk, RootState } from './';
import * as api from '../util/api';
import { ResolveType } from '../util/ts';
import { replace } from '../util/history';

type ResType = ResolveType<ReturnType<typeof api.search>>;

export type SliceState = {
  readonly dirty: boolean;
  readonly query: string;
  readonly verses: {
    readonly [key: string]: {
      readonly [translation: string]: string;
    };
  };
  readonly queries: {
    readonly [key: string]: {
      inFlight: boolean;
      res: Array<{
        key: string;
        topTranslation: string;
        highlights: Array<string>;
      }>;
    };
  };
};

const initialState: SliceState = {
  dirty: false,
  query: '',
  verses: {},
  queries: {},
};

const { actions, reducer } = createSlice({
  name: 'search',
  initialState,
  reducers: {
    startQuery: (state, { payload }: PayloadAction<string>): void => {
      state.dirty = true;
      state.query = payload;

      if (!state.queries[payload]) {
        state.queries[payload] = { inFlight: true, res: [] };
      }
    },
    endQuery: (
      state,
      { payload }: PayloadAction<{ q: string; res: ResType }>,
    ): void => {
      const { q, res } = payload;
      const sq = state.queries[q];
      if (sq && res) {
        sq.inFlight = false;
        res.forEach(r => {
          if (!state.verses[r.key]) {
            state.verses[r.key] = r.text;
          }
          sq.res = res.map(r => ({
            key: r.key,
            topTranslation: r.topTranslation,
            highlights: r.highlights,
          }));
        });
      }
    },
    reset: state => {
      state.dirty = false;
      state.query = '';
    },
  },
});

export default reducer;

export const { startQuery, endQuery, reset } = actions;

export const doSearch = (q: string): AppThunk => async (dispatch, getState) => {
  const { search: searchState, offline: offlineState } = getState();

  replace('/', { q });
  dispatch(startQuery(q));

  if (searchState.queries[q]) {
    return;
  }

  const res = await api.search(q, offlineState.enabled);

  dispatch(endQuery({ q, res }));
};

export const doReset = (): AppThunk => async dispatch => {
  dispatch(reset());
  replace();
};

export const useDirty = (): boolean =>
  useSelector(({ search }: RootState) => search.dirty);

export const useLoading = (q = ''): boolean => {
  const query = useSelector(({ search }: RootState) => search.queries[q]);
  return query ? query.inFlight : false;
};

const selectResults = createSelector(
  (state: RootState) => state.search.query,
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

export const useQuery = () =>
  useSelector((state: RootState) => state.search.query);

export const useResults = (): SliceState['queries'][string] | null => {
  const query = useSelector((state: RootState) => selectResults(state));

  if (query) {
    return query;
  }

  return null;
};

export const useVerse = (key: string): SliceState['verses'][string] => {
  return useSelector((state: RootState) => state.search.verses[key]);
};
