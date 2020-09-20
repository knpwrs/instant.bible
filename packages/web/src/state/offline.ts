import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { useSelector } from 'react-redux';
import { AppThunk, RootState } from './';
import { endQuery } from './search';
import { getIndexBytes, decodeApiResponse } from '../util/api';
import { delLocalBytes } from '../util/index-storeage';
import { OutgoingData } from '../util/local-search-worker';

export type SliceState = {
  readonly enabled: boolean;
  readonly indexBytesProgress: number;
  readonly initialized: boolean;
  readonly loading: boolean;
  readonly worker: Worker | null;
  readonly error: boolean;
};

const initialState: SliceState = {
  enabled: false,
  indexBytesProgress: 0,
  initialized: false,
  loading: false,
  worker: null,
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
    setInitialized: (state, { payload }: PayloadAction<Worker>) => {
      state.initialized = true;
      state.worker = payload;
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
    const { default: SearchWorker } = await import(
      'worker-loader!../util/local-search-worker'
    );
    const worker = new SearchWorker();
    const job = getIndexBytes();
    job.onProgress((p) => dispatch(indexBytesProgress(p)));
    const bytes = await job;
    worker.postMessage({ cmd: 'init', bytes });
    worker.onmessage = (msg) => {
      const data: OutgoingData = msg.data;
      if (data.cmd === 'search') {
        const res = decodeApiResponse(data.res);
        dispatch(
          endQuery({
            q: data.q,
            res: res,
          }),
        );
      } else if (data.cmd === 'init') {
        dispatch(setInitialized(worker));
      }
    };
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
