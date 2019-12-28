import * as React from 'react';
import { storiesOf } from '@storybook/react';
import { text as textKnob } from '@storybook/addon-knobs';
import { H5, Text } from './typography';

storiesOf('elements/typography', module).add('basic', () => {
  const text = textKnob('text', 'Hello, World!');

  return (
    <div>
      <H5>{text}</H5>
      <Text>{text}</Text>
    </div>
  );
});
