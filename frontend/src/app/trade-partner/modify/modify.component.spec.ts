import { ComponentFixture, TestBed } from '@angular/core/testing';

import { TradePartnerModifyComponent } from './modify.component';

describe('TradePartnerModifyComponent', () => {
  let component: TradePartnerModifyComponent;
  let fixture: ComponentFixture<TradePartnerModifyComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [TradePartnerModifyComponent]
    });
    fixture = TestBed.createComponent(TradePartnerModifyComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
