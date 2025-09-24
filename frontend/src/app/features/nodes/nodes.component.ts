import { ChangeDetectionStrategy, Component } from '@angular/core';

@Component({
  selector: 'mina-nodes',
  templateUrl: './nodes.component.html',
  styleUrls: ['./nodes.component.scss'],
  changeDetection: ChangeDetectionStrategy.OnPush,
  standalone: false,
})
export class NodesComponent {}
