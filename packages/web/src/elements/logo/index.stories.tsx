import * as React from 'react';
import { storiesOf } from '@storybook/react';
import { boolean } from '@storybook/addon-knobs';
import Logo from './';

storiesOf('elements/logo', module).add('basic', () => {
  const alt = boolean('alt', false);

  return <Logo alt={alt} />;
});
