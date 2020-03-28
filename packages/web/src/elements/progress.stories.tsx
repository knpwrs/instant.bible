import * as React from 'react';
import { storiesOf } from '@storybook/react';
import { number } from '@storybook/addon-knobs';
import Progress from './progress';

storiesOf('elements/progress', module).add('basic', () => {
  const percent = number('Progress', 0.42, {
    range: true,
    min: 0,
    max: 1,
    step: 0.01,
  });

  return <Progress value={percent} />;
});
