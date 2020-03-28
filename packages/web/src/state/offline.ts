import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { useSelector } from 'react-redux';
import { AppThunk, RootState } from './';
import { getIndexBytes } from '../util/api';
import { setWasm, delLocalBytes } from '../util/bridge';

export type SliceState = {
  readonly enabled: boolean;
  readonly indexBytesProgress: number;
  readonly initialized: boolean;
  readonly loading: boolean;
  readonly error: boolean;
};

const initialState: SliceState = {
  enabled: false,
  indexBytesProgress: 0,
  initialized: false,
  loading: false,
  error: false,
};

const { actions, reducer } = createSlice({
  name: 'offline',
  initialState,
  reducers: {
    indexBytesProgress: (state, { payload }: PayloadAction<number>) => {
      state.indexBytesProgress = payload;
    },
    setOfflineMode: (state, { payload }: PayloadAction<boolean>) => {
      state.enabled = payload;
    },
    setInitialized: state => {
      state.initialized = true;
    },
    setLoading: (state, { payload }: PayloadAction<boolean>) => {
      state.loading = payload;
    },
    error: (state, { payload }: PayloadAction<boolean>) => {
      state.error = payload;
      state.initialized = false;
      state.indexBytesProgress = 0;
    },
  },
});

export default reducer;

const {
  error,
  setInitialized,
  setLoading,
  setOfflineMode,
  indexBytesProgress,
} = actions;

export const doInitOffline = (enable: boolean): AppThunk => async (
  dispatch,
  getState,
) => {
  dispatch(setOfflineMode(enable));

  if (!enable) {
    await delLocalBytes();
  }

  if (!enable || getState().offline.initialized) {
    return;
  }

  try {
    dispatch(setLoading(true));
    const wasm = await import('../wasm');
    const job = getIndexBytes();
    job.onProgress(p => dispatch(indexBytesProgress(p)));
    const bytes = await job;
    wasm.init(bytes);
    setWasm(wasm);
    dispatch(setInitialized());
  } catch (e) {
    console.error(e);
    dispatch(error(true));
  } finally {
    dispatch(setLoading(false));
  }
};

export const useIndexBytesProgress = () =>
  useSelector(({ offline }: RootState) => offline.indexBytesProgress);

export const useOfflineEnabled = () =>
  useSelector(({ offline }: RootState) => offline.enabled);

export const useLoading = () =>
  useSelector(({ offline }: RootState) => offline.loading);
