import * as React from 'react';
import { storiesOf } from '@storybook/react';
import { text } from '@storybook/addon-knobs';
import Header from './header';

storiesOf('elements/header', module).add('basic', () => {
  const txt = text('text', 'Foo Bar');

  return <Header>{txt}</Header>;
});
