import { combineReducers, configureStore, Action } from '@reduxjs/toolkit';
import { ThunkAction } from 'redux-thunk';
import search from './search';

const rootReducer = combineReducers({
  search,
});

const store = configureStore({ reducer: rootReducer });

export default store;

export type RootState = ReturnType<typeof rootReducer>;
export type AppThunk = ThunkAction<void, RootState, null, Action<string>>;
