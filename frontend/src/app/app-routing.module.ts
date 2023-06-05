import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

import { MainPageComponent } from './main-page/main-page.component';
import { ContactComponent } from './contact/contact.component';
import { LoginComponent } from './login/login.component';
import { ParcelStatusComponent } from './parcel-status/parcel-status.component';
import { ErrorPageComponent } from './error-page/error-page.component';
import { TradeParnerListComponent } from './trade-partner/list/list.component';
import { TradePartnerViewComponent } from './trade-partner/view/view.component';
import { TradePartnerModifyComponent } from './trade-partner/modify/modify.component';

import { authGuard } from './auth.guard';

import { Role } from './models/role';

const routes: Routes = [
  { path: '', component: MainPageComponent },
  { path: 'login', component: LoginComponent },
  { path: 'parcel', component: ParcelStatusComponent },
  { path: 'contact', component: ContactComponent },
  { path: 'tradepartner', component: TradeParnerListComponent, canActivate: [authGuard,], data: { role: Role.Admin } },
  { path: 'tradepartner/:id', component: TradePartnerViewComponent, canActivate: [authGuard,], data: { role: Role.Admin } },
  { path: 'tradepartner/:id/modify', component: TradePartnerModifyComponent, canActivate: [authGuard,], data: { role: Role.Admin } },
  { path: '**', component: ErrorPageComponent },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
