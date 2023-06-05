import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { OnInit } from '@angular/core';
import { Router, ActivatedRoute } from '@angular/router';
import { Location } from '@angular/common'

import { environment } from 'src/environments/environment';
import { TradePartner } from 'src/app/models/trade-parnter';

const httpOptions = {
  withCredentials: true
}

@Component({
  selector: 'trade-partner-modify',
  templateUrl: './modify.component.html',
  styleUrls: ['./modify.component.css'],
  host: { 'class': 'hero-body' }
})
export class TradePartnerModifyComponent implements OnInit {
  trade_partner?: TradePartner;
  trade_partner_id?: number;
  price_list_edit = false

  constructor(
    private http: HttpClient,
    private route: ActivatedRoute,
    private router: Router,
    private location: Location
  ) {
    const routeParams = this.route.snapshot.paramMap;
    this.trade_partner_id = Number(routeParams.get('id'));
  }

  ngOnInit() {
    this.http.get<TradePartner>(`${environment.apiUrl}/tradepartner/${this.trade_partner_id}`, httpOptions)
      .subscribe({
        next: (data: TradePartner) => this.trade_partner = data,
        error: () => this.router.navigate(['/notfound'])
      });
  }

  back(): void {
    this.location.back()
  }

  save(name: string) {
    this.http.put<any>(`${environment.apiUrl}/tradepartner/${this.trade_partner_id}`, { name: name }, httpOptions)
      .subscribe({
        next: () => this.back(),
        error: () => this.router.navigate(['/error'])
      });
  }
}
