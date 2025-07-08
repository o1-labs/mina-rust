import { ChangeDetectionStrategy, Component } from '@angular/core';

@Component({
    selector: 'mina-leaderboard-details',
    templateUrl: './leaderboard-details.component.html',
    styleUrl: './leaderboard-details.component.scss',
    changeDetection: ChangeDetectionStrategy.OnPush,
    standalone: false
})
export class LeaderboardDetailsComponent {

}
