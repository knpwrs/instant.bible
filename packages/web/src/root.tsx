import * as React from 'react';
import { hot } from 'react-hot-loader/root';
import Header from './elements/header';

const App: React.FunctionComponent = () => <Header>Hello, World!</Header>;

export default hot(App);
