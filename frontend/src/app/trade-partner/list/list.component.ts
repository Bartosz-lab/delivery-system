import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';

import { environment } from 'src/environments/environment';
import { TradePartner } from 'src/app/models/trade-parnter';

const httpOptions = {
  withCredentials: true
}

@Component({
  selector: 'app-list',
  templateUrl: './list.component.html',
  styleUrls: ['./list.component.css'],
  host: { 'class': 'hero-body' }
})
export class TradeParnerListComponent {
  list?: TradePartner[];

  constructor(
    private http: HttpClient,
  ) {
    this.http.get<TradePartner[]>(`${environment.apiUrl}/tradepartner/list`, httpOptions)
      .subscribe((data: TradePartner[]) => this.list = data);
  }
}
