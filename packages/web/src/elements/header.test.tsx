import * as React from 'react';
import { axe } from 'jest-axe';
import Header from './header';
import render from '../opt/test-render';

test('a11y', async () => {
  const { container } = render(<Header>Testing</Header>);
  expect(await axe(container.innerHTML)).toHaveNoViolations();
});

test('renders text', () => {
  const { container } = render(<Header>Testing</Header>);
  expect(container).toHaveTextContent('Testing');
});
