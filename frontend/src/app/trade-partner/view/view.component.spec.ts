import { ComponentFixture, TestBed } from '@angular/core/testing';

import { TradePartnerViewComponent } from './view.component';

describe('TradePartnerViewComponent', () => {
  let component: TradePartnerViewComponent;
  let fixture: ComponentFixture<TradePartnerViewComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [TradePartnerViewComponent]
    });
    fixture = TestBed.createComponent(TradePartnerViewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
