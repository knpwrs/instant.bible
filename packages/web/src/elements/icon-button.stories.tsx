import * as React from 'react';
import { storiesOf } from '@storybook/react';
import { action } from '@storybook/addon-actions';

import { faCross } from '@fortawesome/free-solid-svg-icons';
import IconButton from './icon-button';

storiesOf('elements/icon-button', module).add('basic', () => {
  return <IconButton icon={faCross} onClick={action('onClick')} />;
});
