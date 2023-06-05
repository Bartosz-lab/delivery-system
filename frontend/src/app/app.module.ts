import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { HttpClientModule } from '@angular/common/http';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { MainPageComponent } from './main-page/main-page.component';
import { ContactComponent } from './contact/contact.component';
import { LoginComponent } from './login/login.component';
import { ParcelStatusComponent } from './parcel-status/parcel-status.component';
import { ErrorPageComponent } from './error-page/error-page.component';
import { TradeParnerListComponent } from './trade-partner/list/list.component';
import { TradePartnerViewComponent } from './trade-partner/view/view.component';
import { TradePartnerModifyComponent } from './trade-partner/modify/modify.component';
import { TradePartnerAddComponent } from './trade-partner/add/add.component';

@NgModule({
  declarations: [
    AppComponent,
    MainPageComponent,
    ContactComponent,
    LoginComponent,
    ParcelStatusComponent,
    ErrorPageComponent,
    TradeParnerListComponent,
    TradePartnerViewComponent,
    TradePartnerModifyComponent,
    TradePartnerAddComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    FormsModule,
    ReactiveFormsModule,
    HttpClientModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
