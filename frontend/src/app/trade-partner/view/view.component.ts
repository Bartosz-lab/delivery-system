import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { OnInit } from '@angular/core';
import { Router, ActivatedRoute } from '@angular/router';

import { environment } from 'src/environments/environment';
import { TradePartner } from 'src/app/models/trade-parnter';
import { PriceListEmem, ParcelSize, Currency } from 'src/app/models/price-list';

const httpOptions = {
  withCredentials: true
}

@Component({
  selector: 'trade-partner-view',
  templateUrl: './view.component.html',
  styleUrls: ['./view.component.css'],
  host: { 'class': 'hero-body' }
})
export class TradePartnerViewComponent implements OnInit {
  trade_partner?: TradePartner;
  price_list?: PriceListEmem[];
  trade_partner_id?: number;
  ParcelSize = Object.keys(ParcelSize)
  Currency = Object.keys(Currency)
  price_list_edit = false

  constructor(
    private http: HttpClient,
    private route: ActivatedRoute,
    private router: Router,
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
    this.http.get<PriceListEmem[]>(`${environment.apiUrl}/tradepartner/${this.trade_partner_id}/pricelist`, httpOptions)
      .subscribe({
        next: (data: PriceListEmem[]) => {
          this.price_list = data;
          this.price_list.sort((a, b) => a[0] == ParcelSize.S ? -1 : (a[0] == ParcelSize.M && b[0] == ParcelSize.L) ? -1 : 1)
        },
        error: () => this.router.navigate(['/notfound'])
      });
  }

  delete() {
    this.http.delete<any>(`${environment.apiUrl}/tradepartner/${this.trade_partner_id}`, httpOptions)
      .subscribe({
        next: () => this.router.navigate(['/tradepartner']),
        error: () => this.router.navigate(['/error'])
      });
  }

  delete_price(size: ParcelSize) {
    this.http.delete<any>(`${environment.apiUrl}/tradepartner/${this.trade_partner_id}/pricelist/${size}`, httpOptions)
      .subscribe({
        next: () => this.ngOnInit(),
        error: () => this.router.navigate(['/error'])
      });
  }

  update_price(size: any, currency: string, price: string) {
    this.http.post<any>(`${environment.apiUrl}/tradepartner/${this.trade_partner_id}/pricelist/${size}`, { currency: currency, price: price }, httpOptions)
      .subscribe({
        next: () => this.ngOnInit(),
        error: () => this.router.navigate(['/error'])
      });
    this.price_list_edit = false;
  }

  edit_price() {
    this.price_list_edit = true;
  }
}
