import 'jest-axe/extend-expect';
import '@testing-library/jest-dom/extend-expect';

// @ts-ignore: Mocking matchMedia for jest
window.matchMedia = (): unknown => ({ matches: false });
