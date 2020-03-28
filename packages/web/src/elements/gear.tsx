import * as React from 'react';
import {
  FontAwesomeIcon,
  FontAwesomeIconProps,
} from '@fortawesome/react-fontawesome';
import { faCog } from '@fortawesome/free-solid-svg-icons';
import { useTheme } from '../util/theme';

export type Props = Pick<FontAwesomeIconProps, 'size' | 'className'>;

export default ({ size = 'lg', className }: Props) => {
  const theme = useTheme();

  return (
    <FontAwesomeIcon
      className={className}
      icon={faCog}
      color={theme.component.icon}
      size={size}
    />
  );
};
