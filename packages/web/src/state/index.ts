import {
  combineReducers,
  configureStore,
  Action,
  getDefaultMiddleware,
} from '@reduxjs/toolkit';
import { ThunkAction } from 'redux-thunk';
import offline, { doInitOffline } from './offline';
import search from './search';
import { getLocalBytes } from '../util/bridge';

const rootReducer = combineReducers({
  search,
  offline,
});

const store = configureStore({
  reducer: rootReducer,
  middleware: getDefaultMiddleware({
    serializableCheck: false,
  }),
});

export default store;

export type RootState = ReturnType<typeof rootReducer>;
export type AppThunk = ThunkAction<void, RootState, null, Action<string>>;

(async () => {
  const bytes = await getLocalBytes();
  if (bytes) {
    store.dispatch(doInitOffline(true));
  }
})();
