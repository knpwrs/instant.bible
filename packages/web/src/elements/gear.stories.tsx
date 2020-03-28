import * as React from 'react';
import { storiesOf } from '@storybook/react';
import { select } from '@storybook/addon-knobs';
import Gear from './gear';

storiesOf('elements/gear', module).add('basic', () => {
  const size = select(
    'size',
    [
      'xs',
      'lg',
      'sm',
      '1x',
      '2x',
      '3x',
      '4x',
      '5x',
      '6x',
      '7x',
      '8x',
      '9x',
      '10x',
    ],
    '2x',
  );

  return <Gear size={size} />;
});
