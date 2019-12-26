import * as React from 'react';
import { storiesOf } from '@storybook/react';
import { text } from '@storybook/addon-knobs';
import Input from './input';

storiesOf('elements/input', module).add('basic', () => {
  const placeholder = text('placeholder', 'Search');

  return <Input placeholder={placeholder} />;
});
