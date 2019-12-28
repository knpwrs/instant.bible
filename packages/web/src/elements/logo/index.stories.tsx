import * as React from 'react';
import { storiesOf } from '@storybook/react';
import { boolean, number } from '@storybook/addon-knobs';
import Logo from './';

storiesOf('elements/logo', module).add('basic', () => {
  const alt = boolean('alt', false);
  const icon = boolean('icon', false);

  const height = number('height %', 50, {
    range: true,
    min: 20,
    max: 100,
    step: 1,
  });

  return (
    <div style={{ height: `${height}%` }}>
      <Logo alt={alt} icon={icon} />
    </div>
  );
});
