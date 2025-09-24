import { ChangeDetectionStrategy, Component } from '@angular/core';

@Component({
  selector: 'mina-mempool-warnings',
  templateUrl: './mempool-warnings.component.html',
  styleUrls: ['./mempool-warnings.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  standalone: false,
})
export class MempoolWarningsComponent {}
