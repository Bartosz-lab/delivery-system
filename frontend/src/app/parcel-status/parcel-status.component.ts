import { Component } from '@angular/core';
import { HttpClient } from '@angular/common/http';

import { environment } from 'src/environments/environment';
import { Parcel } from '../models/parcel';

@Component({
  selector: 'app-parcel-status',
  templateUrl: './parcel-status.component.html',
  styleUrls: ['./parcel-status.component.css'],
  host: { 'class': 'hero-body' }
})
export class ParcelStatusComponent {
  parcel?: Parcel;

  constructor(
    private http: HttpClient,
  ) {
  }

  find(id: string) {
    this.http.get<Parcel>(`${environment.apiUrl}/parcel/${id}`)
      .subscribe((data: Parcel) => {
        this.parcel = data;
        this.parcel.status_list.reverse();
      });
  }

  back_to_find() {
    this.parcel = undefined;
  }
}
