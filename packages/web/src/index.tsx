import * as React from 'react';
import { render } from 'react-dom';
import * as Sentry from '@sentry/browser';
import Root from './root';

if (process.env.NODE_ENV === 'production') {
  Sentry.init({
    dsn: process.env.SENTRY_DSN,
  });
}

const root = document.getElementById('root');

render(<Root />, root);
