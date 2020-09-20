import {
  combineReducers,
  configureStore,
  Action,
  getDefaultMiddleware,
} from '@reduxjs/toolkit';
import { ThunkAction } from 'redux-thunk';
import offline, { doInitOffline } from './offline';
import search, { doSearch } from './search';
import { selectQuery } from '../util/history';
import { getLocalBytes } from '../util/index-storeage';

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
  const query = selectQuery();
  if (query.q) {
    store.dispatch(doSearch(query.q));
  }

  const bytes = await getLocalBytes();
  if (bytes) {
    store.dispatch(doInitOffline(true));
  }
})();
