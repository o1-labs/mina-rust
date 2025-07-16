import { ChangeDetectionStrategy, Component } from '@angular/core';

@Component({
    selector: 'mina-network',
    templateUrl: './network.component.html',
    styleUrls: ['./network.component.scss'],
    changeDetection: ChangeDetectionStrategy.OnPush,
    standalone: false
})
export class NetworkComponent {
}
