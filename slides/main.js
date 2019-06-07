const bespoke = require('bespoke');
const bespokeKeys = require('bespoke-keys');

window.addEventListener('keydown', event => {
  if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
    event.preventDefault();
  }
});

bespoke
  .from('body', [
    bespokeKeys('vertical'),
  ])
  .on('activate', ({ slide }) => {
    slide.scrollIntoView({
      behavior: 'smooth',
      block: 'center',
      inline: 'nearest',
    });

    return false;
  });
