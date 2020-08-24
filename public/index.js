import './scss/home.scss';

import portrait from './assets/self.jpeg';

async function main() {
  const app = await import('../app/pkg');

  app.run_app(new app.Res(portrait));
}

main();
