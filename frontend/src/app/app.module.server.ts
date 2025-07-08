import { bootstrapApplication } from '@angular/platform-browser';
import { provideServerRendering } from '@angular/platform-server';
import { AppComponent } from './app.component';
import { appConfig } from '@app/app.config';

const serverConfig = {
  ...appConfig,
  providers: [
    ...appConfig.providers,
    provideServerRendering(),
  ],
};

export default () => bootstrapApplication(AppComponent, serverConfig);
