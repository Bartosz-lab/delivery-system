import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';
import { Location } from '@angular/common'

import { environment } from 'src/environments/environment';
import { TradePartner } from 'src/app/models/trade-parnter';

const httpOptions = {
  withCredentials: true
}

@Component({
  selector: 'trade-partner-add',
  templateUrl: './add.component.html',
  styleUrls: ['./add.component.css'],
  host: { 'class': 'hero-body' }
})
export class TradePartnerAddComponent {
  constructor(
    private http: HttpClient,
    private router: Router,
    private location: Location
  ) {
  }

  back(): void {
    this.location.back()
  }

  save(name: string) {
    this.http.post<number>(`${environment.apiUrl}/tradepartner`, { name: name }, httpOptions)
      .subscribe({
        next: (data: number) => this.router.navigate(['/tradepartner', data]),
        error: () => this.router.navigate(['/error'])
      });
  }

}
