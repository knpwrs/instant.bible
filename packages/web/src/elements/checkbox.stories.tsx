import * as React from 'react';
import { storiesOf } from '@storybook/react';
import { boolean, text } from '@storybook/addon-knobs';
import { action } from '@storybook/addon-actions';
import Checkbox from './checkbox';

storiesOf('elements/checkbox', module).add('basic', () => {
  const checked = boolean('checked', true);
  const disabled = boolean('disabled', false);
  const label = text('label', 'Label');

  return (
    <Checkbox value={checked} disabled={disabled} onChange={action('onChange')}>
      {label}
    </Checkbox>
  );
});
