import * as React from 'react';
import { axe } from 'jest-axe';
import { render } from '@testing-library/react';
import Header from './header';

test('a11y', async () => {
  const { container } = render(<Header>Testing</Header>);
  expect(await axe(container.innerHTML)).toHaveNoViolations();
});

test('renders text', () => {
  const { container } = render(<Header>Testing</Header>);
  expect(container).toHaveTextContent('Testing');
});
