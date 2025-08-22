import { useEffect } from 'react';
import { useColorMode } from '@docusaurus/theme-common';

export default function MobileThemeToggle() {
  const { colorMode, setColorMode } = useColorMode();

  useEffect(() => {
    // Since we can't directly bind to ::after pseudo-element,
    // we'll listen for clicks on the footer bottom and check position
    const footerBottom = document.querySelector('.footer__bottom');
    if (footerBottom) {
      const handleClick = (e) => {
        const rect = footerBottom.getBoundingClientRect();
        const afterStart = rect.bottom - 60; // approximate height of the ::after element
        if (e.clientY >= afterStart && e.clientY <= rect.bottom) {
          setColorMode(colorMode === 'dark' ? 'light' : 'dark');
        }
      };

      footerBottom.addEventListener('click', handleClick);
      return () => footerBottom.removeEventListener('click', handleClick);
    }
  }, [colorMode, setColorMode]);

  return null;
}