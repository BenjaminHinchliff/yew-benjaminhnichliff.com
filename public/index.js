import './scss/home.scss';

import portrait from './assets/self.jpeg';

import('../app/pkg').then(pkg => pkg.run_app(new pkg.Res(portrait))).catch(console.error);
